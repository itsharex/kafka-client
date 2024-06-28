<script setup lang="ts">
import { RouterView } from "vue-router";
import Navbar from "./components/Navbar.vue";
</script>
<template>
  <div class="relative flex flex-col h-screen overflow-hidden">
    <Navbar />
    <div class="flex-1 border-t border-muted overflow-auto">
      <RouterView />
    </div>
  </div>
</template>
