<template>
  <div>
    <v-container grid-list-md>
      <h1 justify-center>Welcome on the Nuxt + Tauri + Vuetify template</h1>
      <t justify-center align-center>This is a basic project with Nuxt, Tauri & Vuetify configured for quick start.</t>
      <br />
      <br />
      <br />
      <v-btn color="primary" @click="incrementCounter">Increment</v-btn>
      <p>Counter: {{ counter }}</p>
    </v-container>
  </div>
</template>

<script lang="ts" setup>

// import { invoke } from '@tauri-apps/api/core';
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

const counter = ref(0);

async function incrementCounter() {
  console.log('Incrementing counter');
  counter.value = await invoke('increment_counter');
}

</script>

<style></style>