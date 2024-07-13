<script setup lang="ts">
import {  onMounted, ref } from "vue";
import { ConsumerGroup, GroupOffset, createConsumerGroup, getConsumerGroups } from "@/lib/kafka";
import GroupList from "@/components/ConsumerGroups/GroupList.vue";
import GroupDetail from "@/components/ConsumerGroups/GroupDetail.vue";
import { ToastProvider, Toaster, useToast } from "@/components/ui/toast";
const loading = ref(false);
const error = ref<string>("");

const consumerGroups= ref<ConsumerGroup[]>([]);

function fetchConsumerGroups() {
  loading.value = true;
  getConsumerGroups()
    .catch((err) => {
      error.value = err;
      throw err;
    })
    .then((data) => {
      consumerGroups.value = data;
      error.value = "";
    })
    .finally(() => {
      loading.value = false;
    });
}

const selectedGroup = ref<ConsumerGroup>();

onMounted(() => fetchConsumerGroups());
</script>
<template>
  <div class="flex h-full">
    <aside class="bg-muted/40 text-foreground max-w-sm overflow-auto flex-none min-w-80">
        <GroupList :groups="consumerGroups" v-model:selected-group="selectedGroup" @refresh="fetchConsumerGroups"  :error="error" />
    </aside>
    <main class="flex-1 h-full overflow-auto">
        <p class="p-2" v-if="selectedGroup == null">
            Please select a consumer group from groups list.
        </p>
        <GroupDetail v-else :group="selectedGroup" />
    </main>
    <div class="grid gap-2">
      <Toaster/>
    </div>
  </div>
</template>