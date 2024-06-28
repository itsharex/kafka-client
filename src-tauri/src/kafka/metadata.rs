use rdkafka::metadata::{Metadata, MetadataBroker, MetadataPartition, MetadataTopic};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]

pub struct ClusterMetadata {
    pub originating_broker_id: i32, 
    pub brokers: Vec<Broker>,
    pub topics: Vec<Topic>,
}
impl ClusterMetadata {
    pub fn from(cluster: &Metadata) -> Self {

        ClusterMetadata {
            originating_broker_id: cluster.orig_broker_id(),
            brokers: cluster.brokers().into_iter().map(|broker| Broker::from(broker)).collect(),
            topics: cluster.topics().into_iter().map(|topic| Topic::from(topic)).collect()
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Broker {
    pub id: i32,
    pub host: String,
    pub port: i32,
}
impl Broker {
    pub fn from(broker: &MetadataBroker) -> Self {
        Broker {
            id: broker.id().into(),
            host: broker.host().into(),
            port: broker.port().into()
        }
    }
}

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
