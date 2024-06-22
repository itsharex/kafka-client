use rdkafka::metadata::{MetadataPartition, MetadataTopic};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Topic {
    pub name: String,
    pub partitions: Vec<Partition>,
}
impl Topic {
    pub fn from(topic: &MetadataTopic) -> Self {
        Topic {
            name: topic.name().to_owned(),
            partitions: topic
                .partitions()
                .into_iter()
                .map(Partition::from)
                .collect(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Partition {
    pub id: i32,
    pub isr: Vec<i32>,
    pub replicas: Vec<i32>,
    pub leader: i32,
}
impl Partition {
    pub fn from(part: &MetadataPartition) -> Self {
        Self {
            id: part.id(),
            leader: part.leader(),
            isr: Vec::from(part.isr()),
            replicas: Vec::from(part.replicas()),
        }
    }
}
