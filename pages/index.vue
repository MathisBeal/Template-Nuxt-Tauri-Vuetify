<template>
  <div style="width: 60vw; height: 60vh; margin: auto;">
    <v-container class="justify-center d-flex flex-column">
      <h1 class="text-center ma-5">Welcome on the Nuxt + Tauri + Vuetify template</h1>
      <p>This is a basic project with Nuxt, Tauri & Vuetify configured for quick start.</p>
      <p>The packet manager is <code>pnpm</code>.</p>
      <v-btn color="primary mt-16" @click="incrementCounter">Increment</v-btn>
      <p class="text-center mt-5">Counter: {{ counter }}</p>
    </v-container>
  </div>
</template>

<script lang="ts" setup>

import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

const counter = ref(0);

async function incrementCounter() {
  counter.value = await invoke('increment_counter');
}

async function resetCounter() {
  counter.value = await invoke('reset_counter');
}

resetCounter();

</script>

<style></style>