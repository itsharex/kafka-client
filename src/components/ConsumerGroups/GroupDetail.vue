<script setup lang="ts">
import { Badge } from "@/components/ui/badge";
import { Card, CardHeader } from "@/components/ui/card";
import CardContent from "@/components/ui/card/CardContent.vue";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { useToast } from "@/components/ui/toast";
import { ConsumerGroup, TopicGroupOffsets, getGroupOffsets } from "@/lib/kafka";
import { ref, watchEffect } from "vue";

const props = defineProps<{
  group: ConsumerGroup 
}>();

const { toast } = useToast();
const toastError = (error: unknown) => {
  let msg = error instanceof Error ? error.message : error+""
  toast({title: "Error!", description: msg, variant: "destructive", duration: 60000});
}

const committedOffsets = ref<TopicGroupOffsets[]>([]);
watchEffect(async () => {
  if (props.group) {
    try {
      committedOffsets.value = [];
      committedOffsets.value = await getGroupOffsets(props.group.name);
    } catch(err) {
      toastError(err);
    }
  }
})
</script>

<template>
  <header class="px-4 py-2 bg-background sticky top-0 flex items-center justify-between z-10">
    <div>
      <h2 class="text-xl font-bold mb-2 flex items-center space-x-2">
        <span>{{ group?.name }}</span>
        <Badge v-text="group?.state"></Badge>
      </h2>
      <p class="flex space-x-4 text-sm uppercase tracking-wide text-muted-foreground">
        <span><b class="font-semibold">Type:</b> {{ group?.protocol_type }}</span>
        <span><b class="font-semibold">Protocol:</b> {{ group?.protocol }}</span>
        <span><b class="font-semibold">Active Members:</b> {{ group?.members.length }}</span>
      </p>
    </div>
    <div>
    </div>
  </header>

  <main class="p-4 space-y-6">
    <!-- Member Assignments -->
    <Card>
      <CardHeader class="border-b">
        <h3 class="text-xl font-semibold flex items-center space-x-2">
          <span>Active Members</span> 
          <Badge class="text-xs">{{ group?.members.length }}</Badge>
        </h3>
      </CardHeader>
      <CardContent class="p-4">
        <Table v-if="(group?.members.length || 0) > 0">
          <TableHeader>
            <TableRow>
              <TableHead>Consumer Id</TableHead>
              <TableHead class="w-32">Host</TableHead>
              <TableHead class="w-64">Client Id</TableHead>
              <TableHead>Assignments</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="member in group?.members" :key="member.id">
              <TableCell v-text="member.id"></TableCell>
              <TableCell v-text="member.client_host"></TableCell>
              <TableCell v-text="member.client_id"></TableCell>
              <TableCell class="flex flex-col space-y-2 items-start">
                <Badge v-for="assignment in member.assignments" v-text="`${assignment.topic}(${assignment.partitions})`"></Badge>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
        <p v-else>There are no active consumers in this group.</p>
      </CardContent>
    </Card>

    <!-- Group Offsets -->
    <Card>
      <CardHeader class="border-b">
        <h3 class="text-xl font-semibold flex items-center space-x-2">
          <span>Committed Offsets</span> 
          <Badge class="text-xs">{{ committedOffsets.length }}</Badge>
        </h3>
      </CardHeader>
      <CardContent class="p-4 space-y-4">
        <div v-if="committedOffsets.length > 0" v-for="groupOffsets in committedOffsets" :key="groupOffsets.topic">
          <h4 class="text-lg font-semibold" v-text="groupOffsets.topic"></h4>
          <Table class="max-h-96c">
            <TableHeader>
              <TableRow>
                <TableHead class="w-16">Partition #</TableHead>
                <TableHead class="w-64">Beginning</TableHead>
                <TableHead class="w-64">End</TableHead>
                <TableHead class="w-64">Current</TableHead>
                <TableHead class="w-64">Lag</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow v-for="partitionOffsets in groupOffsets.partitions" :key="groupOffsets.topic + partitionOffsets.partition">
                <TableCell v-text="partitionOffsets.partition"></TableCell>
                <TableCell v-text="partitionOffsets.startOffset"></TableCell>
                <TableCell v-text="partitionOffsets.endOffset"></TableCell>
                <TableCell v-text="partitionOffsets.currentOffset"></TableCell>
                <TableCell v-text="partitionOffsets.endOffset - partitionOffsets.currentOffset"></TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
        <p v-else>There are no committed Offsets for this group.</p>
      </CardContent>
    </Card>
  </main>
</template>
