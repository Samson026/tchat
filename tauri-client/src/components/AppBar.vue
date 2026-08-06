<template>
	<div
		class="bg-surface w-43 h-screen rounded-lg px-4 py-5 border border-border"
	>
		<h1 class="text-text font-semibold text-3xl tracking-tight">tChat</h1>
		<div class="flex flex-col">
			<p class="text-text opacity-50 mt-10 text-xl">Chats:</p>
			<UserBtn
				v-for="user in state.chats_data"
				:key="user.id"
				:user="user"
				class="-ml-2"
			/>
			<button class="bg-primary text-text rounded-2xl w-25 h-8 self-center mt-5 text-center hover:bg-primary-hover"
				@click="newChat"
			>
				New Chat
			</button>
		</div>
	</div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted } from "vue";
import type { User } from "../models/user.ts";
import { useState } from "../stores/state.ts";
import UserBtn from "./UserBtn.vue";
import { useRouter } from "vue-router";

const state = useState();
const router = useRouter()

function newChat() {
	router.push("/home/search")
}

onMounted(async () => {
	if (state.user === null) {
		console.log("User is not logged in")
		return
	}
	const newUsers = await invoke<User[]>("get_chats", {
		userId: state.user.id
	});
	newUsers.forEach(user => {
		if (!state.chats_id.has(user.id)) {
			state.chats_id.add(user.id)
			state.chats_data.push(user)
		}
	});
});
</script>
