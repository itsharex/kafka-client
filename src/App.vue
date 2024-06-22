<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import TopicView from "./TopicView.vue";
import { getCurrent } from "@tauri-apps/api/webviewWindow";
import TopicsMenu from './TopicsMenu.vue'
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
export type ClusterConfig = {
  name: string;
  bootstrap_servers: string[];
};
const cluster = ref<ClusterConfig>();
const topicsList = ref<TopicInfo[]>([]);

function fetchTopicsList() {
  loading.value = true;
  invoke<TopicInfo[]>("get_topics")
    .catch((err) => {
      error.value = err;
      throw err;
    })
    .then((data) => {
      topicsList.value = data;
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
onMounted(() => fetchTopicsList());
onMounted(() => getConfig());
onMounted(() => getCurrent().listen<ClusterConfig>("current-cluster-update", (event) => {
  cluster.value = event.payload;
}))
</script>
<template>
  <div class="relative">
    <div class="flex h-screen">
      <aside class="bg-neutral-100 dark:bg-neutral-900 max-w-sm overflow-auto flex-none">
        <div class="flex items-baseline leading-none py-1 px-2 bg-neutral-200 text-neutral-800 dark:bg-neutral-700 dark:text-neutral-200 border-b border-neutral-300">
          <h4 class="uppercase text-xs tracking-wide font-bold">Cluster: <span class="tracking-normal font-semibold normal-case">{{cluster?.name}} ({{cluster?.bootstrap_servers.join(",")}})</span></h4>
        </div>
        <TopicsMenu :error="error" :topics="topicsList" v-model:selectedTopic="selectedTopic" @refresh="fetchTopicsList"/>
      </aside>
      <main class="bg-white dark:bg-neutral-800 flex-1 h-full overflow-auto">
        <TopicView v-if="selectedTopic" :topic="selectedTopic" />
        <div v-else class="h-full flex items-center justify-center">
          <p>Please select a topic from the list.</p>
        </div>
      </main>
    </div>
  </div>
</template>
