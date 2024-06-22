use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::{Headers, OwnedMessage},
    ClientConfig, Message, TopicPartitionList,
};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, time::Duration};

use crate::kafka::metadata::Topic;

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageEnvelope<K, P> {
    pub key: K,
    pub partition: i32,
    pub offset: i64,
    pub headers: HashMap<String, String>,
    pub payload: P,
    pub timestamp: i64,
}

pub struct KafkaConsumer {
    servers: Vec<String>,
    consumer: StreamConsumer,
    topics_metadata: Option<Vec<Topic>>,
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
            topics_metadata: None,
        }
    }

    pub fn get_topics_metadata(&mut self) -> Result<Vec<Topic>, String> {
        match &self.topics_metadata {
            Some(metadata) => Result::Ok(metadata.to_vec()),
            None => self
                .fetch_metadata()
                .and_then(|meta| Ok(self.update_metadata(meta))),
        }
    }

    fn update_metadata(&mut self, metadata: Vec<Topic>) -> Vec<Topic> {
        self.topics_metadata = Some(metadata.clone());
        metadata
    }

    fn fetch_metadata(&self) -> Result<Vec<Topic>, String> {
        self.consumer
            .fetch_metadata(None, Duration::from_secs(2))
            .map(|data| {
                data.topics()
                    .into_iter()
                    .map(Topic::from)
                    .collect::<Vec<Topic>>()
            })
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
            .get_topics_metadata()
            .map(|items| {
                items.into_iter().find_map(|item| {
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
