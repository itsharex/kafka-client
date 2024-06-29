<script setup lang="ts">
import { ConsumerGroup } from "@/lib/kafka";
import {Badge} from "@/components/ui/badge";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";

defineProps<{
  group?: ConsumerGroup 
}>();
</script>

<template>
  <header class="px-4 py-2 bg-muted shadow sticky top-0 flex items-center justify-between z-10">
    <div>
      <h4 class="text-lg font-semibold" v-if="group" v-text="group?.name"></h4>
      <h6 class="text-sm uppercase tracking-wide text-muted-foreground">Members: {{ group?.members.length }}</h6>
    </div>
  </header>

  <main class="p-4">
    <p>[{{ group?.protocol_type }}: {{ group?.protocol }}] | {{ group?.state }}</p>
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead class="w-10">#</TableHead>
          <TableHead class="w-64">Client Host</TableHead>
          <TableHead class="w-64">Client Id</TableHead>
          <TableHead>Assignments</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="member in group?.members" :key="member.id">
          <TableCell v-text="member.id"></TableCell>
          <TableCell v-text="member.client_host"></TableCell>
          <TableCell v-text="member.client_id"></TableCell>
          <TableCell>
            <Badge  v-for="assignment in member.assignments" v-text="`${assignment.topic}(${assignment.partitions})`"></Badge>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </main>
</template>
