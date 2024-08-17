import { ClusterConfig, getConfig } from "@/lib/config";
import { defineStore } from "pinia";
import { computed, ref } from "vue";

type AppConfig = {
  clusters: ClusterConfig[];
  currentClusterName: string;
};
const DEFAULT_APP_CONFIG: AppConfig = {
  clusters: [],
  currentClusterName: ""
};
export const useAppConfig = defineStore("appConfig", () => {
  const config = ref<AppConfig>(DEFAULT_APP_CONFIG);
  const cluster = computed(() =>
    config.value.clusters.find(c => c.name === config.value.currentClusterName)
  );

  function setCurrentCluster(cluster: ClusterConfig) {
    const existingCluster = config.value.clusters.find(c => c.name === cluster.name);
    if (!existingCluster) {
      config.value.clusters.push(cluster);
    }
    config.value.currentClusterName = cluster.name;
  }

  async function loadConfig() {
    const cluster = await getConfig();
    setCurrentCluster(cluster);
  }

  return { cluster, loadConfig, setCurrentCluster };
});
