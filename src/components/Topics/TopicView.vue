<script setup lang="ts">
import { BrokerInfo, ClusterMetadata, ConfigEntry, PartitionInfo, TopicInfo, getTopicConfigs } from "@/lib/kafka";
import { computed, ref, watchEffect } from "vue";
import {Badge} from "@/components/ui/badge";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";

const props = defineProps<{
  topic: TopicInfo;
  cluster: ClusterMetadata;
}>();

const brokerIdNameMap = computed<Record<string, BrokerInfo>>(
  () => Object.fromEntries(
    props.cluster.brokers.map(broker => [broker.id, broker])
  ) 
);

function isReplicaInSync(partition: PartitionInfo, replica: number) {
  return partition.isr.includes(replica);
}


function getBrokerLabel(brokerId: number): String {
  const idString = brokerId.toString();
  return idString in brokerIdNameMap.value ? `#${idString} - ${brokerIdNameMap.value[idString].host}` : `#${idString}`;
}

const configLoading = ref(false);
const topicConfigs = ref<ConfigEntry[]>([]);
const onlyNonDefaultConfigs = computed(() => topicConfigs.value.filter(item => !item.isDefault && item.source!="Default"))
watchEffect(async () => {
  if (props.topic) {
    try {
      configLoading.value = true;
      topicConfigs.value = await getTopicConfigs(props.topic.name);
    } catch(err) {
      console.error(err)
    } finally {
      configLoading.value = false;
    }
  }
})

</script>

<template>
  <header class="px-4 py-2 bg-muted shadow sticky top-0 flex items-center justify-between z-10">
    <div>
      <h4 class="text-lg font-semibold" v-if="topic" v-text="topic.name"></h4>
      <h6 class="text-sm uppercase tracking-wide text-muted-foreground">Paritions: {{ topic.partitions.length }}</h6>
    </div>
  </header>

  <main class="p-4">
    <div v-if="!configLoading && onlyNonDefaultConfigs.length > 0" class="-m-1 px-1 pt-1 mb-4 flex flex-wrap items-center">
      <strong class="font-bold">configs:</strong> 
      <code v-for="config in onlyNonDefaultConfigs" :key="config.name" class="m-1 px-2 py-1 text-xs rounded bg-muted font-mono">
        <span v-text="config.name"></span>="<span v-text="config.value"></span>"
      </code>
    </div>
    <p v-else-if="configLoading" class="mb-4"><strong class="font-bold">configs:</strong> Loading...</p>
    <p v-else class="mb-4"><strong class="font-bold">configs:</strong>  No additional confiuration found for this topic.</p>
    <h2 class="text-xl font-bold mb-2">Partitions</h2>
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead class="w-10">#</TableHead>
          <TableHead class="w-64">Leader</TableHead>
          <TableHead>Replicas</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="partition in topic.partitions" :key="partition.id">
          <TableCell v-text="partition.id"></TableCell>
          <TableCell v-text="getBrokerLabel(partition.leader)"></TableCell>
          <TableCell>
            <Badge :variant="isReplicaInSync(partition, replica) ? 'outline' : 'destructive'" v-for="replica in partition.replicas" v-text="getBrokerLabel(replica)"></Badge>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </main>
</template>
