<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-slate-900">
    <h1 class="text-4xl font-bold text-white">
      Tailwind is working! and old mate
    </h1>
  </div>
</template>
