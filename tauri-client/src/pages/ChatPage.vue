<template>
	<main class="flex flex-col h-screen">
		<div clas="px-10 py-10"></div>
		<h2 class="text-text m-5">Chatting with: {{ username }}</h2>
		<div class="flex min-h-0 flex-1 flex-col justify-end">
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
				<form class="flex h-full items-center" @submit="submitForm">
					<input
						type="text"
						placeholder="Message"
						class="text-text w-full h-full mx-2"
						v-model="input"
					>
					<p v-if="errors.input" class="text-error text-sm">
						{{ errors.input }}
					</p>
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
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import ChatMessage from "../components/ChatMessage.vue";
import type { Message, User } from "../models/user.ts";
import { NewMessage } from "../models/validation.ts";
import { useNotification } from "../stores/notifications.ts";
import { useState } from "../stores/state.ts";

const route = useRoute();
const state = useState();
const notificationStore = useNotification();

const { defineField, handleSubmit, errors, resetForm } = useForm({
	validationSchema: toTypedSchema(NewMessage),
});

const [input] = defineField("input");

const username = computed(() => {
	return state.chats_data.get(Number(route.params.id))?.username;
});

async function getMessaess() {
	const recv_id: number = Number(route.params.id);
	return await invoke<Message[]>("get_messages", {
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

	await invoke("send", {
		message: msg,
	});

	if (state.messages === null) {
		state.messages = [msg];
	}
	state.messages.push(msg);
}

async function getChats() {
	return invoke<User[]>("get_chats");
}

const submitForm = handleSubmit(async (values) => {
	try {
		await sendMessage(values.input);
		resetForm();
	} catch (error) {
		notificationStore.pushError(String(error));
	}
});

// async function handleSubmit() {
// 	if (inputRef.value === null) return;

// 	await sendMessage(inputRef.value);
// 	inputRef.value = "";
// }

onMounted(async () => {
	if (state.user) {
		state.messages = await getMessaess();
		const newUsers = await getChats();
		newUsers.forEach((user) => {
			if (!state.chats_data.has(user.id)) {
				state.chats_data.set(user.id, user);
			}
		});
		console.log("got message");
		console.log(state.messages);
	}
});
</script>
