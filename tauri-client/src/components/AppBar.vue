<template>
	<div
		class="flex flex-col bg-surface w-43 h-screen rounded-lg px-4 py-5 border border-border"
	>
		<h1 class="text-text font-semibold text-3xl tracking-tight">tChat</h1>
		<div class="flex flex-col flex-1 min-h-0">
			<p class="text-text opacity-50 mt-10 text-xl">Chats:</p>
			<UserBtn
				v-for="[ chatId, chatData ] in state.chats_data"
				:key="chatId"
				:user="chatData.user"
				class="-ml-2"
			/>
			<button
				class="bg-primary text-text rounded-2xl w-25 h-8 self-center mt-5 text-center hover:bg-primary-hover"
				type="button"
				@click="newChat"
			>
				New Chat
			</button>
			<button
				class="bg-secondary text-text rounded-2xl w-25 h-8 self-center text-center hover:bg-primary-hover mt-auto"
				type="button"
				@click="logout"
			>
				Logout
			</button>
		</div>
	</div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted } from "vue";
import { useRouter } from "vue-router";
import type { Chat, ChatData, Message, User } from "../models/user.ts";
import { useNotification } from "../stores/notifications.ts";
import { useState } from "../stores/state.ts";
import UserBtn from "./UserBtn.vue";

const state = useState();
const notificationStore = useNotification();
const router = useRouter();

function newChat() {
	router.push("/home/search");
}

async function logout() {
	try {
		await invoke("disconnect_ws");
		await invoke("logout");
		state.$reset();
		router.push("/");
	} catch (error) {
		console.log(error);
	}
}

async function fetchChatData() {
	try {
		const chats = await invoke<Chat[]>("get_chats");
		for (const chat of chats) {
			const messages = await invoke<Message[]>("get_messages", {
				receiverId: chat.user_id,
			});
			for (const message of messages) {
				console.log("here is a message");
				console.log(message.content);
			}
			const user: User = {
				id: chat.user_id,
				username: chat.username,
			};
			const chatData: ChatData = {
				user,
				id: chat.id,
				messages,
				read_count: chat.read_count,
			};
			state.chats_data.set(user.id, chatData);
			console.log(state.chats_data);
		}
	} catch (error) {
		notificationStore.pushError(String(error));
	}
}

onMounted(async () => {
	// const newUsers = await invoke<User[]>("get_chats");
	// newUsers.forEach((user) => {
	// 	if (!state.chats_data.has(user.id)) {
	// 		state.chats_data.set(user.id, user);
	// 	}
	// });
	await fetchChatData();
});
</script>
