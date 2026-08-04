<template>
  <main class="grid min-h-screen place-items-center bg-slate-900 p-6 text-white">
    <h1 class="text-center text-5xl font-semibold">tChat</h1>
    <div class="h-64 flex w-full max-w-sm rounded-xl bg-white shadow-2xl">
       
        <div class="flex w-full flex-col px-20 py-4">

            <h1 class="text-black text-center text-4xl font-semibold opacity-50 my-4 px-10">Login</h1>

            <input 
                type="text"
                class="w-full max-w-sm rounded-lg border border-slate-200 text-black bg-grey my-1 font-extralight"
                placeholder="Username"
                v-model="username"
            >
            <input type="password"
                class="w-full max-w-sm rounded-lg border border-slate-300 text-black bg-grey my-1 font-extralight"
                placeholder="Password"
            >
            <button
                class="bg-sky-500 w-30 self-center rounded-lg hover:bg-sky-700 my-2"
                @click="login"
                type="button"
            >
            Login
            </button>
        </div>
        
    </div>
  </main>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import type { User } from "../models/user";
import { router } from "../router";
import { useState } from "../stores/state";

const username = ref("");
const user = ref<User | null>(null);
const state = useState();

async function login() {
	user.value = await invoke<User>("login", {
		username: username.value,
	});

	if (user.value.username) {
		state.user = user.value;
		router.push("/home");
	}
}
</script>
