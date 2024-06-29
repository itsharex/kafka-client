use byteorder::BigEndian;
use rdkafka::{
    consumer::{Consumer, StreamConsumer}, groups::{GroupInfo, GroupList, GroupMemberInfo}, message::{Headers, OwnedMessage}, util::Timeout, ClientConfig, Message, TopicPartitionList
};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, io::Cursor, time::Duration};
use byteorder::{ReadBytesExt};

use super::{metadata::ClusterMetadata, util::read_str};

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageEnvelope<K, P> {
    pub key: K,
    pub partition: i32,
    pub offset: i64,
    pub headers: HashMap<String, String>,
    pub payload: P,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberAssignment {
    pub topic: String,
    pub partitions: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerGroup {
    name: String,
    state: String,
    protocol: String,
    protocol_type: String,
    members: Vec<ConsumerGroupMember>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct ConsumerGroupMember {
    id: String,
    client_id: String,
    client_host: String,
    metadata: Vec<u8>,
    assignments: Vec<MemberAssignment>
}

impl ConsumerGroup {
    pub fn from(group: &GroupInfo) -> Self {
        let members: Vec<ConsumerGroupMember> = group.members().into_iter()
            .map(|member| ConsumerGroupMember::from(member))
            .filter_map(|res| res.ok())
            .collect();

        ConsumerGroup {
            name: group.name().to_string(),
            state: group.state().to_string(),
            protocol: group.protocol().to_string(),
            protocol_type: group.protocol_type().to_string(),
            members
        }
    }
}

impl ConsumerGroupMember {
    pub fn from(member: &GroupMemberInfo) -> Result<Self, String> {
        let member_metadata = member.metadata().map(|meta| meta.to_vec()).unwrap_or(vec![]);
        ConsumerGroupMember::parse_member_assignment(member.assignment())
            .map(|mem_assignments| ConsumerGroupMember {
                id: member.id().to_string(),
                client_id: member.client_id().to_string(),
                client_host: member.client_host().to_string(),
                metadata: member_metadata,
                assignments: mem_assignments
            })
    }

    fn parse_member_assignment(payload: Option<&[u8]>) -> Result<Vec<MemberAssignment>, String> {
        if payload.is_none(){
            return Ok(vec![]);
        }

        let mut cursor = Cursor::new(payload.unwrap());
        let _version = cursor.read_i16::<BigEndian>()
            .map_err(|e| format!("{}", e))?;
        let assign_len = cursor.read_i32::<BigEndian>()
            .map_err(|e| format!("{}", e))?;
        let mut assigns = Vec::with_capacity(assign_len as usize);
        for _ in 0..assign_len {
            let topic = read_str(&mut cursor)
                .map_err(|e| format!("{}", e))?
                .to_string();
            let partition_len = cursor.read_i32::<BigEndian>()
                .map_err(|e| format!("{}", e))?;
            let mut partitions = Vec::with_capacity(partition_len as usize);
            for _ in 0..partition_len {
                let partition = cursor.read_i32::<BigEndian>()
                    .map_err(|e| format!("{}", e))?;
                partitions.push(partition);
            }
            assigns.push(MemberAssignment { topic, partitions })
        }
        Ok(assigns)
    }
}
pub struct KafkaConsumer {
    servers: Vec<String>,
    consumer: StreamConsumer,
    metadata: Option<ClusterMetadata>,
}

impl KafkaConsumer {
    pub fn connect(bootstrap_servers: Vec<String>) -> Self {
        Self {
            servers: bootstrap_servers.clone(),
            consumer: ClientConfig::new()
                .set("bootstrap.servers", bootstrap_servers.join(","))
                .set("group.id", "runtime")
                .set("enable.auto.commit", "false")
                .create::<StreamConsumer>()
                .expect(
                    format!(
                        "Error while connecting to servers {}",
                        bootstrap_servers.join(",")
                    )
                    .as_str(),
                ),
            metadata: None,
        }
    }

    pub fn get_metadata(&mut self) -> Result<ClusterMetadata, String> {
        match &self.metadata {
            Some(metadata) => Result::Ok(metadata.to_owned()),
            None => self
                .fetch_metadata()
                .and_then(|meta| Ok(self.update_metadata(meta))),
        }
    }

    pub fn get_groups_list(&self) -> Result<Vec<ConsumerGroup>, String> {
        self.consumer.fetch_group_list(None, Timeout::After(Duration::from_secs(300)))
        .map_err(|err| err.to_string())
        .map(|group_list| group_list.groups().into_iter().map(|group| ConsumerGroup::from(group)).collect())
    }

    fn update_metadata(&mut self, metadata: ClusterMetadata) -> ClusterMetadata {
        self.metadata = Some(metadata.clone());
        metadata
    }

    fn fetch_metadata(&self) -> Result<ClusterMetadata, String> {
        self.consumer
            .fetch_metadata(None, Duration::from_secs(2))
            .map(|data| ClusterMetadata::from(&data))
            .map_err(|err| err.to_string())
    }

    pub async fn consume_by_timestamps(
        &mut self,
        topic: &str,
        start: i64,
        end: i64,
    ) -> Result<MessageEnvelope<String, String>, String> {
        let mut start_offset_timestamp_list = TopicPartitionList::new();
        let topic_partitions = self
            .get_metadata()
            .map(|metadata| {
                metadata.topics.into_iter().find_map(|item| {
                    if item.name == topic {
                        Some(item.partitions)
                    } else {
                        None
                    }
                })
            })
            .expect("failed to fetch metadata");

        if matches!(topic_partitions, None) {
            return Err("Invalid topic name".to_owned());
        }

        for partition in topic_partitions.unwrap() {
            start_offset_timestamp_list
                .add_partition_offset(topic, partition.id, rdkafka::Offset::Offset(start))
                .unwrap();
        }

        let start_offsets_list = self
            .consumer
            .offsets_for_times(start_offset_timestamp_list, Duration::from_secs(2))
            .unwrap();

        self.consumer
            .assign(&start_offsets_list)
            .expect("Failed to assign the Offsets!");

        self.get_next_message().await
    }

    pub async fn get_next_message(&self) -> Result<MessageEnvelope<String, String>, String> {
        self.consumer
            .recv()
            .await
            .map(|x| x.detach())
            .map(Self::convert_message)
            .map_err(|err| err.to_string())
    }

    fn convert_message(message: OwnedMessage) -> MessageEnvelope<String, String> {
        let key = message
            .key()
            .map(String::from_utf8_lossy)
            .unwrap_or_default()
            .to_string();
        let timestamp = message.timestamp().to_millis().unwrap_or_default();
        let headers = message
            .headers()
            .map(|h| {
                let mut map = HashMap::with_capacity(h.count());
                for i in 0..h.count() {
                    let (key, val) = h
                        .get(i)
                        .map(|(key, val)| {
                            (key.to_string(), String::from_utf8_lossy(val).to_string())
                        })
                        .unwrap();
                    map.insert(key, val);
                }
                map
            })
            .unwrap_or_default();
        let payload = message
            .payload()
            .map(String::from_utf8_lossy)
            .map(|v| v.to_string())
            .unwrap_or_default();
        MessageEnvelope {
            key,
            partition: message.partition(),
            offset: message.offset(),
            headers,
            payload,
            timestamp,
        }
    }
}
