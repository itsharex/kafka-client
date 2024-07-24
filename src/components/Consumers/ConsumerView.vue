<script setup lang="ts">
import { ref, watch, watchEffect } from "vue";
import { GroupOffset, JsonMessageEnvelope, MessageEnvelope, consumeFromTopicWithinTimeRange, stopConsumer } from "@/lib/kafka";
import { cn, getLang, jsonText } from "@/lib/utils";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Button } from "@/components/ui/button";
import { useToast } from "@/components/ui/toast";
import { listen } from "@tauri-apps/api/event";
import { Label } from "../ui/label";
import { type DateValue, today, DateFormatter, getLocalTimeZone } from "@internationalized/date";
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { CalendarIcon } from "lucide-vue-next";
import { Calendar } from "../ui/calendar";

const props = defineProps<{
  topic: string;
}>();

const messages = ref<JsonMessageEnvelope[]>([]);

const modalMessage = ref<JsonMessageEnvelope | undefined>();
const isOpen = ref(false);
const openModal = (message: JsonMessageEnvelope) => {
  modalMessage.value = message;
  isOpen.value = true;
}
const cleanUpModal = () => {modalMessage.value = undefined;}

const {toast} = useToast();

const isConsuming = ref(false);
const consumerId = ref<string>();
const consumerSubscriptionCleanUp = ref<() => void>();
const stop = async () => {
  if (consumerId.value) {
    await stopConsumer(consumerId.value);
    consumerId.value = undefined;
    consumerSubscriptionCleanUp.value && consumerSubscriptionCleanUp.value();
    isConsuming.value = false;
  }
}

const df = new DateFormatter(getLang(), {
  dateStyle: 'long',
});
const offsetType = ref<GroupOffset["type"]>("End");
const offsetTimestamp = ref<DateValue>(today(getLocalTimeZone()));

function fetchMessage() {
  if (isConsuming.value) {
    return;
  }
  const now = Date.now(); // ms epoch
  const duration = 24 * 60 * 60 * 1000 // 24hrs duration in ms
  isConsuming.value = true;
  consumerId.value = undefined;
  consumeFromTopicWithinTimeRange(props.topic, [now-duration, now+(duration/4)])
    .then(async (event_channel) => {
      consumerId.value = event_channel;
    })
    .catch((err) => {
      toast({title: "Error", description: ""+err, variant:"destructive"});
      isConsuming.value = false;
    });
}

watchEffect(async () => {
  if (consumerId.value) { // subscribe on obtaining the consumerId
    messages.value = [];
    const unlisten = await listen<MessageEnvelope|null>(consumerId.value, (evt) => {
      console.log("Listened", {evt});
      if (!evt.payload) { // Tombstone Payload
        unlisten();
        isConsuming.value = false;
        return;
      }
      messages.value.push({...evt.payload, payloadJson: jsonText(evt.payload.payload)});
    });
    consumerSubscriptionCleanUp.value = () => {
      unlisten();
      isConsuming.value = false;
    }
    return consumerSubscriptionCleanUp.value;
  }
});

watch(() => props.topic, (newTopic, oldTopic) => {
  if (newTopic) {
    // topic changed, cleanup.
    messages.value = [];
  }

  if (oldTopic!=newTopic) {
    (isConsuming.value || consumerId.value !=null) && stop();
  }
});
</script>

<template>
  <header class="px-4 py-2 bg-background sticky top-0 flex items-center justify-between z-10">
    <div>
      <h2 class="text-xl font-bold mb-2 flex items-center space-x-2">
        <span v-text="topic"></span>
      </h2>
      <p>Consumer will start consuming from 24hrs ago till now and until 6hrs from now.</p>
    </div>
    <Dialog>
      <DialogTrigger as-child>
        <Button :disabled="isConsuming">Consume</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Start New Consumer</DialogTitle>
          <DialogDescription>Choose confiugration to start consuming from the topic `{{ props.topic }}`
          </DialogDescription>
        </DialogHeader>
        <form id="consumer-creation-form" @submit.prevent="() => fetchMessage()">
          <div class="grid gap-2">
            <Label for="group_id">
              Offset
            </Label>
            <div class="flex space-x-2">
              <Select v-model:model-value="offsetType">
                <SelectTrigger class="w-[180px]">
                  <SelectValue placeholder="Choose Offset Type" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="End">
                      Latest
                    </SelectItem>
                    <SelectItem value="Beginning">
                      Earliest
                    </SelectItem>
                    <SelectItem value="Offset">
                      By Timestamp
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
              <Popover v-if="offsetType === 'Offset'">
                <PopoverTrigger as-child>
                  <Button variant="outline" :class="cn(
                      'w-[280px] justify-start text-left font-normal',
                      !offsetTimestamp && 'text-muted-foreground',
                    )">
                    <CalendarIcon class="mr-2 h-4 w-4" />
                    {{ offsetTimestamp ? df.format(offsetTimestamp.toDate(getLocalTimeZone())) : "Pick a date" }}
                  </Button>
                </PopoverTrigger>
                <PopoverContent class="w-auto p-0">
                  <Calendar v-model:model-value="offsetTimestamp" initial-focus />
                </PopoverContent>
              </Popover>
            </div>
          </div>
        </form>
        <DialogFooter>
          <DialogClose>Cancel</DialogClose>
          <Button form="consumer-creation-form" type="submit">Start</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
    <Button v-if="isConsuming" @click="stop">
      Stop
    </Button>
  </header>

  <main class="p-4">
    <ul class="space-y-4">
      <li v-for="currentMessage in messages" :key="currentMessage.payload" class="bg-neutral-100 shadow rounded-md p-2">
        <ul v-if="Object.keys(currentMessage.headers).length > 0" class="space-x-2 flex items-center flex-wrap mb-2">
          <li v-for="key in Object.keys(currentMessage.headers)">
            <span v-text="key" class="px-2 py-1 text-xs bg-neutral-300 rounded-full"></span>
          </li>
        </ul>
        <pre @click="() => openModal(currentMessage)"
          class="truncate mb-2"><code v-text="currentMessage.payload"></code></pre>
        <footer class="-mx-2 -mb-2 px-2 py-1 border-t border-neutral-300 text-xs flex justify-between">
          <div class="space-x-3 flex items-baseline">
            <p>Key: "{{ currentMessage.key }}"</p>
            <p>Partition: {{ currentMessage.partition }}</p>
            <p>Offset: {{ currentMessage.offset }}</p>
          </div>
          <div class="space-x-2 flex items-baseline">
            <p>{{ new Date(currentMessage.timestamp).toLocaleDateString() }}</p>
            <p>{{ new Date(currentMessage.timestamp).toLocaleTimeString() }}</p>
          </div>
        </footer>
      </li>
    </ul>
    <Dialog v-model:open="isOpen" @update:open="(value) => {if (!value) {cleanUpModal()}}">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            Message
          </DialogTitle>
          <DialogDescription>Received on {{modalMessage?.partition}}@{{ modalMessage?.offset }}</DialogDescription>
        </DialogHeader>
        <div class="mt-2 p-3 rounded-lg bg-neutral-100 w-full overflow-auto">
          <pre
            class="text-sm text-gray-500"><code v-text="JSON.stringify(modalMessage?.payloadJson, null, 2)"></code></pre>
        </div>
      </DialogContent>
    </Dialog>
  </main>
</template>
