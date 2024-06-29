import { invoke } from "@tauri-apps/api/core";

export function getConfig(): Promise<ClusterConfig> {
  return invoke<ClusterConfig>("get_current_cluster");
}

export type ClusterConfig = {
  name: string;
  bootstrap_servers: string[];
};
