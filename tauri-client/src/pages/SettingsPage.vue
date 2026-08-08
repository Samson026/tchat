<template>
    <main class="bg-background h-screen p-20 ">
        <BackBtn class="absolute top-6 left-6"/>
        <h2 class="text-text">Settings</h2>
        <div class="grid grid-cols-2 gap-1 p-20 bg-surface rounded-2xl">
            <h2 class="text-text self-center">Server Address</h2>
            <input
            type="text"
            class="text-text bg-secondary rounded-xs p-1"
            :placeholder="serverAddrRef.toString()"
            v-model="serverAddrRef"
            >
            </input>
            <button
            type="submit"
            class="text-text bg-primary rounded-2xl w-20 opacity-55 hover:opacity-100"
            @click="update_settings"
            >
                Save
            </button>
        </div>
    </main>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import BackBtn from '../components/BackBtn.vue';
import { useState } from '../stores/state';
import { invoke } from '@tauri-apps/api/core';
import { Settings } from '../models/user';

const state = useState()
const serverAddrRef = ref(state.settings?.server_address ?? "Could not load settings...")

async function update_settings() {
  const settings: Settings = {
    server_address: serverAddrRef.value
  }
  try {
    await invoke("update_settings", {
      settings: settings
    })
  } catch (error) {
    console.log(error)
  }
}

onMounted(async () => {
  if (state.settings === null) {
    state.settings = await invoke("get_settings")
    console.log(serverAddrRef.value)
    serverAddrRef.value = state.settings?.server_address ?? "Could not load settings..."
  }
})
</script>
