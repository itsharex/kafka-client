import { RouteRecordRaw, createMemoryHistory, createRouter } from "vue-router";
import Topics from "./pages/Topics.vue";
import ConsumerGroups from "./pages/ConsumerGroups.vue";
import Consumer from "./pages/Consumer.vue";

const routes: RouteRecordRaw[] = [
  { path: "/", component: Topics },
  { path: "/consumer-groups", component: ConsumerGroups },
  { path: "/consumer", component: Consumer }
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
