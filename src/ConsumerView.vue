<script setup lang="ts">
import { ref,onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrent } from "@tauri-apps/api/webviewWindow";
import {
  TransitionRoot,
  TransitionChild,
  Dialog,
  DialogPanel,
  DialogTitle,
} from '@headlessui/vue';
import { Button } from "./components/ui/button";
import { TopicInfo } from "./pages/Home.vue";

const props = defineProps<{
  topic: TopicInfo;
}>();

const loading = ref(false);
const error = ref("");
type MessageEnvelope = {
  key: string;
  offset: number;
  partition: number;
  timestamp: number;
  payload: string;
  headers: Record<string, string>;
};

type JsonMessageEnvelope = MessageEnvelope & {payload: number | boolean | string | Record<string, unknown>};

const messages = ref<JsonMessageEnvelope[]>([]);

const modalMessage = ref<JsonMessageEnvelope | undefined>();
const isOpen = ref(false);
const openModal = (message: JsonMessageEnvelope) => {
  console.log("open Modal");
  modalMessage.value = message;
  isOpen.value = true;
}
const cleanUpModal = () => {modalMessage.value = undefined;}
const closeModal = () => {isOpen.value = false;}

function jsonText(obj: string) {
  try {
    return JSON.parse(obj);
  } catch(error) {
    return obj;
  }
}

function fetchMessage() {
  loading.value = true;
  error.value = "";
  invoke<MessageEnvelope>("consume_topic_by_timestamp", {
    topic: props.topic?.name,
    start: Date.now() - 12 * 60 * 60 * 1000,
    end: Date.now(),
  })
    .catch((err) => {
      error.value = err;
      throw err;
    })
    .then((msg) => {
      messages.value = [{...msg, payload: jsonText(msg.payload)}];
      error.value = "";
    })
    .finally(() => {
      loading.value = false;
    });
}
onMounted(() =>
  getCurrent().listen<MessageEnvelope>("new_message", (evt) => {
    messages.value.push({...evt.payload, payload: jsonText(evt.payload.payload)});
  })
);
</script>

<template>
  <header class="px-4 py-2 bg-neutral-100 shadow sticky top-0 flex items-center justify-between">
    <div>
      <h4 class="text-lg font-semibold" v-if="topic" v-text="topic.name"></h4>
      <h6 class="text-sm uppercase tracking-wide text-neutral-700">Paritions: {{ topic.partitions.length }}</h6>
    </div>
    <Button type="button" @click="() => fetchMessage()">Start Consuming</Button>
  </header>

  <main class="p-4">
    <p v-if="loading">Loading...</p>
    <ul v-else-if="!error" class="space-y-4">
      <li v-for="currentMessage in messages" :key="currentMessage.payload" class="bg-neutral-100 shadow rounded-md p-2">
        <ul v-if="Object.keys(currentMessage.headers).length > 0" class="space-x-2 flex items-center flex-wrap mb-2">
          <li v-for="key in Object.keys(currentMessage.headers)">
            <span v-text="key" class="px-2 py-1 text-xs bg-neutral-300 rounded-full"></span>
          </li>
        </ul>
        <pre @click="() => openModal(currentMessage)" class="truncate mb-2"><code v-text="currentMessage.payload"></code></pre>
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
    <p v-else="error" class="bg-error-200 text-error-700 px-4 py-2" v-text="error"></p>
    <TransitionRoot appear :show="isOpen" as="template" :on-after-leave="cleanUpModal">
      <Dialog as="div" @close="closeModal" class="relative z-10">
        <TransitionChild
          as="template"
          enter="duration-300 ease-out"
          enter-from="opacity-0"
          enter-to="opacity-100"
          leave="duration-200 ease-in"
          leave-from="opacity-100"
          leave-to="opacity-0"
        >
          <div class="fixed inset-0 bg-neutral-900/70" />
        </TransitionChild>

        <div class="fixed inset-0 overflow-y-auto">
          <div
            class="flex min-h-full items-center justify-center p-4 text-center"
          >
            <TransitionChild
              as="template"
              enter="duration-300 ease-out"
              enter-from="opacity-0 scale-95"
              enter-to="opacity-100 scale-100"
              leave="duration-200 ease-in"
              leave-from="opacity-100 scale-100"
              leave-to="opacity-0 scale-95"
            >
              <DialogPanel
                class="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all"
              >
                <DialogTitle
                  as="h3"
                  class="text-lg font-medium leading-6 text-gray-900"
                >
                  Message on {{modalMessage?.partition}}@{{ modalMessage?.offset }}
                </DialogTitle>
                <div class="mt-2 p-3 rounded-lg bg-neutral-100">
                  <pre class="text-sm text-gray-500"><code v-text="JSON.stringify(modalMessage?.payload, null, 4)"></code></pre>
                </div>

                <div class="mt-4">
                  <button
                    type="button"
                    class="inline-flex justify-center rounded-md border border-transparent bg-primary-100 px-2 py-1 text-sm font-medium text-primary-900 hover:bg-primary-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-primary-500 focus-visible:ring-offset-2"
                    @click="closeModal"
                  >
                    Close
                  </button>
                </div>
              </DialogPanel>
            </TransitionChild>
          </div>
        </div>
      </Dialog>
    </TransitionRoot>
  </main>
</template>
