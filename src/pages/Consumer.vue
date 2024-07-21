<script setup lang="ts">
import { computed, onMounted, reactive, ref } from "vue";
import { getCurrent } from "@tauri-apps/api/webviewWindow";
import TopicList from "@/components/Topics/TopicList.vue";
import TopicView from "@/components/Topics/TopicView.vue";
import { ClusterMetadata, TopicInfo, getClusterMetadata } from "@/lib/kafka";
import { ClusterConfig, getConfig } from "@/lib/config";
import ConsumerView from "@/components/Consumers/ConsumerView.vue";
const loading = ref(false);
const error = ref<string>("");

const cluster = ref<ClusterConfig>();

let clusterMetadata = reactive<ClusterMetadata>({originating_broker_id: 0, topics: [], brokers: []});
const topicsList = computed(() => clusterMetadata.topics );
function fetchClusterMetadata() {
  loading.value = true;
  getClusterMetadata()
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
onMounted(() => fetchClusterMetadata());

const selectedTopic = ref<TopicInfo>();
onMounted(() => getConfig().then(config => cluster.value = config));
onMounted(() => getCurrent().listen<ClusterConfig>("current-cluster-update", (event) => {
  cluster.value = event.payload;
}))
</script>
<template>
  <div class="flex h-full">
    <aside class="bg-muted/40 text-foreground max-w-sm overflow-auto flex-none min-w-80">
        <TopicList :topics="topicsList" v-model:selected-topic="selectedTopic" @refresh="fetchClusterMetadata"  :error="error" />
    </aside>
    <main class="flex-1 h-full overflow-auto">
        <p class="p-2" v-if="selectedTopic == null">
            Please select a topic from topic list.
        </p>
        <ConsumerView v-else :topic="selectedTopic.name" />
    </main>
  </div>
</template>