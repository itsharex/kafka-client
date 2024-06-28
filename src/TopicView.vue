<script setup lang="ts">
import { BrokerInfo, ClusterMetadata, PartitionInfo, TopicInfo } from "./pages/Topics.vue";
import { computed } from "vue";
import {Badge} from "./components/ui/badge";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./components/ui/table";

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

</script>

<template>
  <header class="px-4 py-2 bg-neutral-100 shadow sticky top-0 flex items-center justify-between">
    <div>
      <h4 class="text-lg font-semibold" v-if="topic" v-text="topic.name"></h4>
      <h6 class="text-sm uppercase tracking-wide text-neutral-700">Paritions: {{ topic.partitions.length }}</h6>
    </div>
  </header>

  <main class="p-4">
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>#</TableHead>
          <TableHead>Leader</TableHead>
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
