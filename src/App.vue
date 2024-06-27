<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { RouterView } from "vue-router";
import { getCurrent } from "@tauri-apps/api/webviewWindow";
import TopicsMenu from './TopicsMenu.vue'
import Navbar from "./components/Navbar.vue";
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
  <div class="relative flex flex-col h-screen">
    <Navbar />
    <div class="flex flex-1 border-t border-muted">
      <aside class="bg-muted text-foreground max-w-sm overflow-auto flex-none">
        <div class="flex items-baseline leading-none py-1 px-2">
          <h4 class="uppercase text-xs tracking-wide font-bold">
            Cluster: 
            <span class="tracking-normal font-semibold normal-case">
              {{cluster?.name}} ({{cluster?.bootstrap_servers.join(",")}})
            </span>
          </h4>
        </div>
        
      </aside>
      <RouterView />
    </div>
  </div>
</template>
