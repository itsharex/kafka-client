use std::{borrow::Borrow, ffi::CStr, ptr::slice_from_raw_parts, time::Duration};

use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication, TopicResult}, bindings::{rd_kafka_AdminOptions_new, rd_kafka_ListOffsets, rd_kafka_ListOffsetsResultInfo_topic_partition, rd_kafka_ListOffsets_result_infos, rd_kafka_event_ListOffsets_result, rd_kafka_event_destroy, rd_kafka_event_error, rd_kafka_event_error_string, rd_kafka_queue_destroy, rd_kafka_queue_new, rd_kafka_queue_poll}, client::{Client, DefaultClientContext}, config::FromClientConfig, consumer::{BaseConsumer, CommitMode, Consumer}, error::IsError, metadata::Metadata, topic_partition_list::TopicPartitionListElem, util::Timeout, ClientConfig, ClientContext, Offset, TopicPartitionList
};

use crate::core::commands::GroupOffset;

fn create_admin_client(bootstrap_servers: Vec<String>, config: ClientConfig) -> AdminClient<DefaultClientContext> {
    AdminClient::from_config(
        config.to_owned()
            .set("bootstrap.servers", bootstrap_servers.join(","))
            .set("group.id", "runtime")
            .set("enable.auto.commit", "false")
    )
    .expect("Error while creating admin client")
}

fn create_base_consumer(bootstrap_servers: Vec<String>, config: &mut ClientConfig) -> BaseConsumer {
    config.to_owned()
        .set("bootstrap.servers", bootstrap_servers.join(","))
        .create()
        .expect("Error creating client")
}

pub async fn create_topic(
    bootstrap_servers: Vec<String>,
    topic: &str,
    partitions: i32,
    replication_factor: i32,
    topic_config: Vec<(&str, &str)>,
    options: Option<AdminOptions>,
) -> TopicResult {
    let client = create_admin_client(bootstrap_servers, ClientConfig::default());
    let new_topic = NewTopic {
        config: topic_config,
        name: topic,
        num_partitions: partitions,
        replication: TopicReplication::Fixed(replication_factor),
    };
    
    let out = client
        .create_topics(
            vec![new_topic.borrow()],
            options.unwrap_or_default().borrow(),
        )
        .await
        .and_then(|val| Ok(val.first().unwrap().to_owned()))
        .expect("Could not get Result");
    return out.clone();
}

pub async fn create_consumer_group(
    bootstrap_servers: Vec<String>,
    group_id: &str,
    topics: Vec<&str>,
    initial_offset: GroupOffset,
) -> Result<(), String> {
    let client = create_base_consumer(bootstrap_servers, ClientConfig::default()
        .set("group.id", group_id)
        .set("enable.auto.offset.store", "false")
    );

    // TODO: check if group id exists already.
    // if (check_group_with_topics(group, topics)) {
    //      return Err(format!("{} group offsets already exists {}"))
    // }


    let offsets = get_topics_offsets(client.client(), topics, initial_offset.into(), Offset::End)?;

    client.commit(&offsets, CommitMode::Sync)
        .map_err(|err| err.to_string())
}

fn get_topics_offsets<C: ClientContext>(client: &Client<C>, topics: Vec<&str>, offset: Offset, fallback_offset: Offset) -> Result<TopicPartitionList, String> {
    // Fetch all topic/paritions with latest metadata.
    let mut tpl = TopicPartitionList::new();
    for topic_name in &topics {
        let meta = client.fetch_metadata(Some(topic_name), Timeout::After(Duration::from_secs(5)))
            .map_err(|err| err.to_string())?;
        let topics_meta  = meta.topics().first().filter(|t| t.partitions().len() > 0);
        if let Some(topic) = topics_meta {
            let start_partitions = 0;
            let total_partitions = topic.partitions().into_iter().map(|p| p.id()).max().unwrap();
            tpl.add_partition_range(topic.name(), start_partitions, total_partitions);
        }
    }
    let _ = tpl.set_all_offsets(offset).map_err(|err| err.to_string())?;

    let offset_list = unsafe {
        get_topic_partition_offsets(client, &tpl)?
    };
    
    let invalid_offsets: Vec<TopicPartitionListElem> = offset_list.elements().into_iter()
        .filter(|el| matches!(el.offset(), Offset::Invalid))
        .collect();
   
    if invalid_offsets.len() == 0 {
        return Ok(offset_list);
    }
   
    // To commit offset maybe just the Offset::End fallback should work. TODO: Confirm this!
    if let Offset::Offset(_) = offset {
        tpl.set_all_offsets(fallback_offset).map_err(|err| err.to_string())?;
        let fallback_map = unsafe {
            get_topic_partition_offsets(client, &tpl)
            .map(|list| list.to_topic_map())?
        };
        
        let mut updated_offset_list = offset_list.clone();
        for tpl_el in invalid_offsets {
            let (topic, partition) = (tpl_el.topic(), tpl_el.partition());
            let fallback_offset = fallback_map.get(&(topic.to_string(), partition)).unwrap_or_else(|| &Offset::Invalid);
            updated_offset_list.set_partition_offset(topic, partition, *fallback_offset)
                .map_err(|err| err.to_string())?;
        }
        return Ok(updated_offset_list);
     } 

    Ok(offset_list)
}


pub unsafe fn get_topic_partition_offsets<C: ClientContext>(client: &Client<C>, topic_partition_list: &TopicPartitionList) -> Result<TopicPartitionList, String> {
    let native_client = client.native_ptr();
    let q = rd_kafka_queue_new(native_client);
    let o = rd_kafka_AdminOptions_new(
        native_client,
        rdkafka::types::RDKafkaAdminOp::RD_KAFKA_ADMIN_OP_LISTOFFSETS,
    );
    rd_kafka_ListOffsets(native_client, topic_partition_list.ptr(), o, q);

    let event = rd_kafka_queue_poll(q, 5000);
    if event.is_null() {
        return Err(format!("No event received from rd_kafka_queue_poll"));
    }
    let result = rd_kafka_event_ListOffsets_result(event);
    if result.is_null() {
        rd_kafka_event_destroy(event);
        rd_kafka_queue_destroy(q);
        return Err(format!("No result received from rd_kafka_event_ListOffsets_result"));
    }

    let err = rd_kafka_event_error(event);
    if err.is_error() {
        let msg = rd_kafka_event_error_string(event);
        let err_str = CStr::from_ptr(msg).to_string_lossy().into_owned();
        return Err(format!("Error: {}", err_str))
    }

    let mut len: usize = 0;
    let raw = &mut len as *mut usize;
    let list_offset_result_infos = rd_kafka_ListOffsets_result_infos(result, raw);
    if list_offset_result_infos.is_null() {
        rd_kafka_event_destroy(event);
        rd_kafka_queue_destroy(q);
        return Err(format!("Failed to get list_offset_result_infos"));
    }

    let s = &*slice_from_raw_parts(list_offset_result_infos, len);
    let mut new_tpl = TopicPartitionList::new();
    for inf in s {
        let top_part = *rd_kafka_ListOffsetsResultInfo_topic_partition(*inf);
        let topic_str = CStr::from_ptr(top_part.topic).to_string_lossy().into_owned();
        let res = new_tpl.add_partition_offset(
            &topic_str,
            top_part.partition,
            rdkafka::Offset::Offset(top_part.offset),
        );
        if let Err(err) = res {
            eprintln!("Error setting offset: {:#?}", err);
        }
    }

    rd_kafka_event_destroy(event);
    rd_kafka_queue_destroy(q);
    Ok(new_tpl)
}
