use std::{borrow::Borrow, collections::HashMap};

use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication, TopicResult},
    client::DefaultClientContext,
    config::FromClientConfig,
    ClientConfig,
};

fn create_admin_client(bootstrap_servers: Vec<String>, config: ClientConfig) -> AdminClient<DefaultClientContext> {
    AdminClient::from_config(
        config.to_owned()
            .set("bootstrap.servers", bootstrap_servers.join(","))
            .set("group.id", "runtime")
            .set("enable.auto.commit", "false")
    )
    .expect("Error while creating admin client")
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
