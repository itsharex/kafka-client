use std::collections::HashMap;
use rdkafka::Offset;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::block_on;
use tauri::{Emitter,  State};

use crate::core::config::{AppConfiguration, ClusterConfig};

use crate::kafka::admin::{self, get_topic_configs, ConfigProperty};
use crate::kafka::consumer::{ConsumerGroup, ConsumerGroupOffsetDescription, KafkaConsumer, MessageEnvelope};
use crate::kafka::metadata::ClusterMetadata;

#[tauri::command]
pub fn get_current_cluster(app_config: State<AppConfiguration>) -> ClusterConfig {
    app_config.config.lock().unwrap().default_cluster_config()
}

#[tauri::command(async)]
pub fn get_topics(app_config: State<AppConfiguration>) -> Result<ClusterMetadata, String> {
    KafkaConsumer::connect(
        app_config
            .config
            .lock()
            .unwrap()
            .default_cluster_config()
            .bootstrap_servers,
    )
    .get_metadata()
}

#[tauri::command(async)]
pub async fn fetch_topic_configs(app_config: State<'_, AppConfiguration>, topic: &str) -> Result<Vec<ConfigProperty>, String> {
    let bootstrap_servers = app_config
    .config
    .lock()
    .unwrap()
    .default_cluster_config()
    .bootstrap_servers;

    admin::get_topic_configs(bootstrap_servers, topic).await
}

#[tauri::command(async)]
pub async fn delete_topic(app_config: State<'_, AppConfiguration>, topic: &str) -> Result<String, String> {
    let bootstrap_servers = app_config
        .config
        .lock()
        .unwrap()
        .default_cluster_config()
        .bootstrap_servers;

    admin::delete_topic(bootstrap_servers, topic).await
}

#[tauri::command(async)]
pub fn get_group_offsets(app_config: State<AppConfiguration>, group_name: String) -> Result<Vec<ConsumerGroupOffsetDescription>, String> {
    let servers = app_config.config.lock().unwrap()
        .default_cluster_config().bootstrap_servers;
    
    KafkaConsumer::connect_config(HashMap::from([
       ("bootstrap.servers".to_owned(), servers.join(",")),
       ("group.id".to_owned(), group_name.clone())
    ])).get_committed_offsets()
}

#[tauri::command(async)]
pub fn get_groups(app_config: State<AppConfiguration>) -> Result<Vec<ConsumerGroup>, String> {
    KafkaConsumer::connect(
        app_config
            .config
            .lock()
            .unwrap()
            .default_cluster_config()
            .bootstrap_servers,
    )
    .get_groups_list()
}



#[tauri::command(async)]
pub async fn create_topic(
    app_config: State<'_, AppConfiguration>,
    topic: &str,
    partitions: i32,
    config: Vec<(&str, &str)>
) -> Result<String, String> {
    let future = admin::create_topic(
        app_config
            .config
            .lock()
            .unwrap()
            .default_cluster_config()
            .bootstrap_servers,
        topic,
        partitions,
        1,
        config,
        None,
    );
    let result = future.await;

    match result {
        Ok(out) => Ok(out),
        Err((data, _err_code)) => Err(data),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag="type", content="content")]
pub enum GroupOffset {
    Beginning,
    End,
    Tail(i64),
    Offset(i64),
}
impl Into<Offset> for GroupOffset {
    fn into(self) -> Offset {
        match self {
            Self::End => Offset::End,
            Self::Beginning => Offset::Beginning,
            Self::Tail(offset) => Offset::OffsetTail(offset),
            Self::Offset(timestamp) => Offset::Offset(timestamp)
        }
    }
}


#[tauri::command(async)]
pub async fn create_group_offsets(app_config: State<'_, AppConfiguration>, group_id: &str, topics: Vec<&str>, initial_offset: GroupOffset) -> Result<(), String> {
    let servers = app_config.config.lock().unwrap()
        .default_cluster_config().bootstrap_servers;

    admin::create_consumer_group(servers, group_id, topics, initial_offset).await
}

#[tauri::command(async)]
pub async fn consume_topic_by_timestamp(
    window: tauri::WebviewWindow,
    app_config: State<'_, AppConfiguration>,
    topic: &str,
    start: i64,
    end: i64,
) -> Result<MessageEnvelope<String, String>, String> {
    println!("Consume started");
    let mut stream = KafkaConsumer::connect(
        app_config
            .config
            .lock()
            .unwrap()
            .default_cluster_config()
            .bootstrap_servers,
    );

    let result = stream.consume_by_timestamps(topic, start, end).await;

    println!("Spawning Thread to consume messages");
    std::thread::spawn(move || loop {
        let message = block_on(stream.get_next_message()).expect("Could not get next message");
        if message.timestamp > end {
            break;
        }
        window.emit("new_message", message)
            .expect("Failed to emit event");
    });

    result
}
