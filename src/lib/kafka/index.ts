import { invoke } from "@tauri-apps/api/core";

// Kafka Admin
export type CreateTopicRequest = { topic: string; partitions: number; config: string[] };
export function createTopic(topicCreateRequest: CreateTopicRequest): Promise<undefined> {
  return invoke("create_topic", topicCreateRequest);
}

// Topics & Broker Metadata
export type PartitionInfo = {
  id: number;
  isr: number[];
  replicas: number[];
  leader: number;
};
export type TopicInfo = {
  name: string;
  partitions: PartitionInfo[];
};

export type BrokerInfo = {
  id: number;
  host: string;
  port: number;
};

export type ClusterMetadata = {
  originating_broker_id: number;
  topics: TopicInfo[];
  brokers: BrokerInfo[];
};
export function getClusterMetadata(): Promise<ClusterMetadata> {
  return invoke<ClusterMetadata>("get_topics");
}

// Consumer Groups Metadata
export type MemberAssignment = {
  topic: string;
  partitions: number;
};

export type ConsumerGroup = {
  name: string;
  state: string;
  protocol: string;
  protocol_type: string;
  members: ConsumerGroupMember[];
};

export type ConsumerGroupMember = {
  id: string;
  client_id: string;
  client_host: string;
  metadata: Uint8Array;
  assignments: MemberAssignment[];
};

export function getConsumerGroups(): Promise<ConsumerGroup[]> {
  return invoke<ConsumerGroup[]>("get_groups");
}