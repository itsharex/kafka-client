import { RouteRecordRaw, createMemoryHistory, createRouter } from "vue-router";
import Topics from "./pages/Topics.vue";

const routes: RouteRecordRaw[] = [{ path: "/", component: Topics }];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
