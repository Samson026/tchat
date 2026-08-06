<template>
	<main class="h-full">
		<div clas="px-10 py-10"></div>
		<div class="flex flex-col h-full justify-end">
			<div class="flex flex-col col h-full justify-end px-10 py-10">
				<ChatMessage
					v-for="(message, index) in state.messages"
					:key="index"
					:message="message"
					:primary="message.sender_id === state.user?.id"
					:class="message.sender_id === state.user?.id ? 'self-end' : 'self-start'"
				/>
			</div>
			<div
				class="bg-surface w-full h-20 border border-border rounded-xl px-2 py-2"
			>
				<form class="flex h-full items-center" @submit.prevent="handleSubmit">
					<input
						type="text"
						placeholder="Message"
						class="text-text w-full h-full mx-2"
						v-model="inputRef"
					>
					<button
						class="text-text bg-primary rounded-2xl min-w-15"
						type="submit"
					>
						Send
					</button>
				</form>
			</div>
		</div>
	</main>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import ChatMessage from "../components/ChatMessage.vue";
import type { Message, User } from "../models/user.ts";
import { useState } from "../stores/state.ts";

const route = useRoute();
const state = useState();

const inputRef = ref("");

async function getMessaess(userID: number) {
	const recv_id: number = Number(route.params.id);
	return await invoke<Message[]>("get_messages", {
		senderId: userID,
		receiverId: recv_id,
	});
}

async function sendMessage(message: string) {
	if (!state.user) return;

	const recv_id = Number(route.params.id);

	const msg: Message = {
		sender_id: state.user.id,
		recv_id: recv_id,
		content: message,
	};

	try {
		await invoke("send", {
			message: msg,
		});

		if (state.messages === null) {
			state.messages = [msg];
		}
		state.messages.push(msg);
	} catch (error) {
		console.log(error);
	}
}

async function getChats(userId: number) {
	return invoke<User[]>("get_message", {
		userId: userId
	})
}

async function handleSubmit() {
	if (inputRef.value === null) return;

	await sendMessage(inputRef.value);
	inputRef.value = "";
}

onMounted(async () => {
	if (state.user) {
		await invoke("connect_ws", {
			userId: state.user.id,
		});
		state.messages = await getMessaess(state.user.id);
		state.chats = await getChats(state.user.id);
	}
});
</script>
