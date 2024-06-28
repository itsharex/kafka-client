<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, onMounted, reactive, ref } from "vue";
import { getCurrent } from "@tauri-apps/api/webviewWindow";
import TopicsMenu from "@/TopicsMenu.vue";
import TopicView from "@/TopicView.vue";
import { Computer } from "lucide-vue-next";
const loading = ref(false);
const error = ref<string>("");
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
}

export type ClusterMetadata = {
    originating_broker_id: number;
    topics: TopicInfo[];
    brokers: BrokerInfo[];
}
export type ClusterConfig = {
  name: string;
  bootstrap_servers: string[];
};
const cluster = ref<ClusterConfig>();

let clusterMetadata = reactive<ClusterMetadata>({originating_broker_id: 0, topics: [], brokers: []});
const topicsList = computed(() => clusterMetadata.topics );
function fetchClusterMetadata() {
  loading.value = true;
  invoke<ClusterMetadata>("get_topics")
    .catch((err) => {
      error.value = err;
      throw err;
    })
    .then((data) => {
      clusterMetadata.originating_broker_id = data.originating_broker_id;
      clusterMetadata.topics = data.topics;
      clusterMetadata.brokers = data.brokers;
      error.value = "";
    })
    .finally(() => {
      loading.value = false;
    });
}

function getConfig() {
  invoke<ClusterConfig>("get_current_cluster")
  .then(config => cluster.value = config)
}

const selectedTopic = ref<TopicInfo>();
onMounted(() => fetchClusterMetadata());
onMounted(() => getConfig());
onMounted(() => getCurrent().listen<ClusterConfig>("current-cluster-update", (event) => {
  cluster.value = event.payload;
}))
</script>
<template>
  <div class="flex h-full">
    <aside class="bg-muted/40 text-foreground max-w-sm overflow-auto flex-none min-w-80">
        <TopicsMenu :topics="topicsList" v-model:selected-topic="selectedTopic" @refresh="fetchClusterMetadata"  :error="error" />
    </aside>
    <main class="flex-1 h-full overflow-auto">
        <p class="p-2" v-if="selectedTopic == null">
            Please select a topic from topic list.
        </p>
        <TopicView v-else :cluster="clusterMetadata" :topic="selectedTopic" />
    </main>
  </div>
</template>