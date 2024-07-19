export const allTopicConfigs = [
  {
    name: "cleanup.policy",
    description:
      "The cleanup policy for segments beyond the retention window. 'delete' means discard old segments when their retention time or size is exceeded, 'compact' means retain at most one record per key.",
    type: "string",
    validValues: ["delete", "compact", "delete,compact"],
    defaultValue: "delete",
  },
  {
    name: "compression.type",
    description:
      "Specify the final compression type for a given topic. This configuration accepts the standard compression codecs ('gzip', 'snappy', 'lz4', 'zstd'). It additionally accepts 'uncompressed' which is equivalent to no compression; and 'producer' which means retain the original compression codec set by the producer.",
    type: "string",
    validValues: ["uncompressed", "zstd", "lz4", "snappy", "gzip", "producer"],
    defaultValue: "producer",
  },
  {
    name: "delete.retention.ms",
    description:
      "The time to retain delete markers for log compacted topics. This setting also gives a bound on the time in which a consumer must complete a read if they begin from offset 0 to ensure that they get a valid snapshot of the final stage (otherwise delete markers could be collected before they complete their scan).",
    type: "long",
    validValues: "1ms - Long.MAX_VALUE",
    defaultValue: "86400000 (24 hours)",
  },
  {
    name: "file.delete.delay.ms",
    description: "The time to wait before deleting a file from the filesystem.",
    type: "long",
    validValues: "1ms - Long.MAX_VALUE",
    defaultValue: "60000 (1 minute)",
  },
  {
    name: "flush.messages",
    description:
      "This setting allows specifying an interval at which we will force an fsync of data written to the log. For example, if this setting is set to 1 we will fsync after every message; if it is set to 5 we will fsync after every five messages. In general, it is recommended not to set this and use replication for durability and allow the operating system's background flush capabilities as it is more efficient.",
    type: "long",
    validValues: "1 - Long.MAX_VALUE",
    defaultValue: "9223372036854775807 (Long.MAX_VALUE)",
  },
  {
    name: "flush.ms",
    description:
      "This setting allows specifying a time interval at which we will force an fsync of data written to the log. For example, if this setting is set to 1000 we will fsync after 1000 ms have passed. In general, it is recommended not to set this and use replication for durability and allow the operating system's background flush capabilities as it is more efficient.",
    type: "long",
    validValues: "1ms - Long.MAX_VALUE",
    defaultValue: "9223372036854775807 (Long.MAX_VALUE)",
  },
  {
    name: "index.interval.bytes",
    description:
      "This setting controls how frequently Kafka adds an index entry to its offset index. The default setting adds an entry approximately every 4096 bytes. More indexing allows reads to jump closer to the exact position in the log but makes the index larger.",
    type: "int",
    validValues: "0 - Integer.MAX_VALUE",
    defaultValue: "4096",
  },
  {
    name: "leader.replication.throttled.replicas",
    description:
      "A list of replicas for which log replication should be throttled on the leader side. The list should describe a set of replicas in the form [PartitionId]:[BrokerId],... or alternatively the wildcard '*' can be used to throttle all replicas for this topic.",
    type: "list",
    validValues: "List of replica IDs",
    defaultValue: "empty",
  },
  {
    name: "max.compaction.lag.ms",
    description:
      "The maximum amount of time a message will remain ineligible for compaction in the log. Only applicable for logs that are being compacted.",
    type: "long",
    validValues: "1ms - Long.MAX_VALUE",
    defaultValue: "9223372036854775807 (Long.MAX_VALUE)",
  },
  {
    name: "max.message.bytes",
    description:
      "This is the largest record batch size allowed by Kafka. If this is increased and there are consumers older than 0.10.2, the consumers' fetch size must also be increased so that they can fetch record batches this large.",
    type: "int",
    validValues: "1 - Integer.MAX_VALUE",
    defaultValue: "1048588 (1 MB)",
  },
  {
    name: "message.format.version",
    description:
      "Specify the message format version the broker will use to append messages to the logs. The value should be a valid ApiVersion. Some examples are: 0.8.2, 0.9.0.0, 0.10.0, 0.10.0-IV1, 0.10.1-IV0, etc.",
    type: "string",
    validValues: "ApiVersion",
    defaultValue: "Broker's inter.broker.protocol.version",
  },
  {
    name: "message.timestamp.difference.max.ms",
    description:
      "The maximum difference allowed between the timestamp when a broker receives a message and the timestamp specified in the message. If message.timestamp.type=CreateTime, this difference is the maximum allowed age of a message. If message.timestamp.type=LogAppendTime, this difference is the maximum allowed delay for a message. If this difference is exceeded, the message will be rejected.",
    type: "long",
    validValues: "1ms - Long.MAX_VALUE",
    defaultValue: "9223372036854775807 (Long.MAX_VALUE)",
  },
  {
    name: "message.timestamp.type",
    description:
      "Define whether the timestamp in the message is the message creation time or the log append time. The value should be either CreateTime or LogAppendTime.",
    type: "string",
    validValues: ["CreateTime", "LogAppendTime"],
    defaultValue: "CreateTime",
  },
  {
    name: "min.cleanable.dirty.ratio",
    description:
      "This ratio bounds the maximum amount of log duplication that Kafka will tolerate in a log before cleaning will commence.",
    type: "double",
    validValues: "0.0 - 1.0",
    defaultValue: "0.5",
  },
  {
    name: "min.compaction.lag.ms",
    description:
      "The minimum time a message will remain uncompacted in the log. Only applicable for logs that are being compacted.",
    type: "long",
    validValues: "0ms - Long.MAX_VALUE",
    defaultValue: "0",
  },
  {
    name: "min.insync.replicas",
    description:
      "When a producer sets acks to 'all' (or '-1'), min.insync.replicas specifies the minimum number of replicas that must acknowledge a write for the write to be considered successful. If this minimum cannot be met, then the producer will raise an exception (either NotEnoughReplicas or NotEnoughReplicasAfterAppend). When used together, min.insync.replicas and acks allow you to enforce greater durability guarantees. A typical scenario would be to create a topic with a replication factor of 3, set min.insync.replicas to 2, and produce with acks of 'all'. This will ensure that the producer raises an exception if a majority of replicas do not receive a write.",
    type: "int",
    validValues: "1 - Integer.MAX_VALUE",
    defaultValue: "1",
  },
  {
    name: "preallocate",
    description:
      "Should pre allocate file when create new segment? If true, will pre allocate file when create new segment.",
    type: "boolean",
    validValues: ["true", "false"],
    defaultValue: "false",
  },
  {
    name: "retention.bytes",
    description:
      "This configuration controls the maximum size of the log before Kafka will begin deleting old log segments to free up space if we are using the 'delete' retention policy. By default there is no size limit only a time limit.",
    type: "long",
    validValues: "-1, 1 - Long.MAX_VALUE",
    defaultValue: "-1",
  },
  {
    name: "retention.ms",
    description:
      "This configuration controls the maximum time we will retain a log before we will discard old log segments to free up space if we are using the 'delete' retention policy. This represents an SLA on how soon consumers must read their data. If set to -1, no time limit is applied.",
    type: "long",
    validValues: "-1, 1ms - Long.MAX_VALUE",
    defaultValue: "604800000 (7 days)",
  },
  {
    name: "segment.bytes",
    description:
      "This is the segment file size for the log. Retention and cleaning is always done a file at a time so a larger segment size means fewer files but less granular control over retention.",
    type: "int",
    validValues: "14 bytes - 1GB",
    defaultValue: "1073741824 (1 GB)",
  },
  {
    name: "segment.index.bytes",
    description:
      "This is the size of the index that maps offsets to file positions. We preallocate this index file and shrink it only after log rolls. You generally should not need to change this setting.",
    type: "int",
    validValues: "4 KB - 1 GB",
    defaultValue: "10485760 (10 MB)",
  },
  {
    name: "segment.jitter.ms",
    description:
      "The maximum jitter to subtract from log segment roll time to avoid thundering herds of segment rolling.",
    type: "long",
    validValues: "0ms - Long.MAX_VALUE",
    defaultValue: "0",
  },
  {
    name: "segment.ms",
    description:
      "This configuration controls the period of time after which Kafka will force the log to roll even if the segment file isn't full to ensure that retention can delete or compact old data.",
    type: "long",
    validValues: "1ms - Long.MAX_VALUE",
    defaultValue: "604800000 (7 days)",
  },
  {
    name: "unclean.leader.election.enable",
    description:
      "Indicates whether to enable replicas not in the ISR set to be elected as leader as a last resort, even though doing so may result in data loss.",
    type: "boolean",
    validValues: ["true", "false"],
    defaultValue: "false",
  },
];
