<script setup lang="ts">
import { BrokerInfo, ClusterMetadata, ConfigEntry, PartitionInfo, TopicInfo, alterTopicConfigs, getTopicConfigs } from "@/lib/kafka";
import { computed, ref, watchEffect } from "vue";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { PlusIcon } from "lucide-vue-next";
import { allTopicConfigs } from "@/lib/kafka/topicConfigs";
import {Badge} from "@/components/ui/badge";
import { DialogClose, DialogContent, DialogDescription, DialogHeader, DialogTrigger, DialogFooter, Dialog } from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Input } from "@/components/ui/input";

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

const configLoading = ref(false);
const topicConfigs = ref<ConfigEntry[]>([]);
const onlyNonDefaultConfigs = computed(() => topicConfigs.value.filter(item => !item.isDefault && item.source!="Default"))
const showAddConfigsDialog = ref(false);
const newConfig = ref(allTopicConfigs.at(0)?.name ?? "");
const selectedNewConfig = computed(() => allTopicConfigs.find(c => c.name === newConfig.value));
const newConfigValue = ref("");
watchEffect(() => {
  if (selectedNewConfig.value) {
    newConfigValue.value = selectedNewConfig.value.defaultValue
  }
});
const addTopicConfigs = () => {
  const existingConfigs = Object.fromEntries(onlyNonDefaultConfigs.value.map(i => [i.name, i.value ?? ""]));
  const newConfigs = {[newConfig.value]: newConfigValue.value};
  alterTopicConfigs(props.topic.name, {...existingConfigs, ...newConfigs})
    .then(() => {
      showAddConfigsDialog.value = false;
      newConfig.value = allTopicConfigs[0].name;
      newConfigValue.value = allTopicConfigs[0].defaultValue;
      loadCurrentTopicConfigs()
    });
}
const removeTopicConfigs = (configName: string) => {
  const removedConfigs = Object.fromEntries(onlyNonDefaultConfigs.value.filter(i => i.name != configName)
  .map(i => [i.name, i.value || ""]));
  alterTopicConfigs(props.topic.name, removedConfigs)
    .then(() => loadCurrentTopicConfigs());
}
const loadCurrentTopicConfigs = async () => {
  try {
      configLoading.value = true;
      topicConfigs.value = await getTopicConfigs(props.topic.name);
    } catch(err) {
      console.error(err)
    } finally {
      configLoading.value = false;
    }
}
watchEffect(async () => {
  if (props.topic) {
    loadCurrentTopicConfigs();
  }
})

</script>

<template>
  <header class="px-4 py-2 bg-muted shadow sticky top-0 flex items-center justify-between z-10">
    <div>
      <h4 class="text-lg font-semibold" v-if="topic" v-text="topic.name"></h4>
      <h6 class="text-sm uppercase tracking-wide text-muted-foreground">Paritions: {{ topic.partitions.length }}</h6>
    </div>
  </header>

  <main class="p-4">
    <div v-if="!configLoading && onlyNonDefaultConfigs.length > 0" class="-m-1 px-1 pt-1 mb-4 flex flex-wrap items-center">
      <strong class="font-bold">configs:</strong> 
      <Badge variant="outline" v-for="config in onlyNonDefaultConfigs" :key="config.name" class="m-1 font-mono inline-flex items-baseline space-x-2">
        <span v-text="`${config.name}=${config.value}`"></span>
        <Button as="a" variant="link" size="xs" class="p-0 hover:text-red-500 cursor-pointer h-auto" @click="() => removeTopicConfigs(config.name)">x</Button>
      </Badge>
      <Dialog v-model:open="showAddConfigsDialog">
        <DialogTrigger as-child>
          <Button variant="default" size="xs" class="h-6 w-6 p-0 rounded-full">
            <PlusIcon class="block w-3 h-3" />
          </Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              Add Topic Config
            </DialogTitle>
            <DialogDescription>
              Add configuration options for : <code class="relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold" v-text="topic.name"></code>
            </DialogDescription>
          </DialogHeader>
          <form class="grid gap-5">
            <div class="grid gap-2">
              <Label for="topic-config-name">Config</Label>
              <Select id="topic-config-name" v-model="newConfig">
                <SelectTrigger>
                  <SelectValue class="text-left" placeholder="Select a config" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem v-for="configOpt in allTopicConfigs" :key="configOpt.name" :value="configOpt.name">
                    <p v-text="configOpt.name"></p>
                    <p class="max-w-[450px] truncate text-muted-foreground" v-text="configOpt.description"></p>
                  </SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label for="topic-config-value">Value</Label>
              <div class="space-y-2">
                <Input id="topic-config-value" type="text" v-model="newConfigValue" :default-value="selectedNewConfig?.defaultValue"/>
                <div class="text-xs space-y-1" v-if="selectedNewConfig">
                  <p class="text-muted-foreground"><strong class="font-bold">Type:</strong> <span v-text="selectedNewConfig.type"></span></p>
                  <p class="text-muted-foreground"><strong class="font-bold">Valid Values:</strong> <span v-text="selectedNewConfig.validValues"></span></p>
                  <p class="text-muted-foreground"><strong class="font-bold">Default:</strong> <span v-text="selectedNewConfig.defaultValue"></span></p>
                </div>
              </div>
            </div>
          </form>
          <DialogFooter>
            <DialogClose as-child>
              <Button variant="secondary">Cancel</Button>
            </DialogClose>
            <Button @click="addTopicConfigs">Add Config</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
    <p v-else-if="configLoading" class="mb-4"><strong class="font-bold">configs:</strong> Loading... </p>
    <p v-else class="mb-4"><strong class="font-bold">configs:</strong>  
      No additional confiuration found for this topic. 
      <Button variant="outline" size="xs" @click="addTopicConfigs"><PlusIcon class="block w-4 h-4" /></Button>
    </p>
    <h2 class="text-xl font-bold mb-2">Partitions</h2>
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead class="w-10">#</TableHead>
          <TableHead class="w-64">Leader</TableHead>
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
