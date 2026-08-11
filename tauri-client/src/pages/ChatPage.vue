<template>
	<main class="flex flex-col h-screen">
		<div clas="px-10 py-10"></div>
		<h2 class="text-text m-5">Chatting with: {{ username }}</h2>
		<div class="flex min-h-0 flex-1 flex-col justify-end">
			<div
				class="flex flex-col col h-full justify-end px-10 py-10 overflow-y-auto"
			>
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
				<form class="flex h-full w-full items-center gap-2" @submit="submitForm">
					<label
						class="flex h-full w-14 shrink-0 cursor-pointer items-center justify-center rounded-2xl bg-secondary"
					>
						<CameraIcon class="h-6 w-6 text-text" />
						<input
							type="file"
							class="hidden"
							accept="image/*"
							v-on:change="onImageSelected"
						>
					</label>

					<input
						type="text"
						placeholder="Message"
						class="h-full min-w-0 flex-1 text-text"
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
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import ChatMessage from "../components/ChatMessage.vue";
import type { Attachment, Message, User } from "../models/user.ts";
import { NewMessage } from "../models/validation.ts";
import { useNotification } from "../stores/notifications.ts";
import { useState } from "../stores/state.ts";
import { CameraIcon } from "lucide-vue-next";

const route = useRoute();
const state = useState();
const notificationStore = useNotification();

const { defineField, handleSubmit, errors, resetForm } = useForm({
	validationSchema: toTypedSchema(NewMessage),
});

const [input] = defineField("input");

const selectedFile = ref<File | null>(null);

const username = computed(() => {
	return state.chats_data.get(Number(route.params.id))?.username;
});

function onImageSelected(event: Event) {
  console.log("inside on image select")
  const input = event.target as HTMLInputElement;
  selectedFile.value = input.files?.[0] ?? null;

}

async function getMessaess() {
	const recv_id: number = Number(route.params.id);
	return await invoke<Message[]>("get_messages", {
		receiverId: recv_id,
	});
}

async function sendMessage(message: string, attachment: Attachment | null) {
	if (!state.user) return;

	const recv_id = Number(route.params.id);

	const msg: Message = {
		sender_id: state.user.id,
		recv_id: recv_id,
		content: message,
		attachment: attachment?.id ?? null
	};

	await invoke("send", {
		message: msg,
	});

	if (state.messages === null) {
		state.messages = [msg];
	}
	state.messages.push(msg);
}

async function uploadImage(file: File) {
  console.log("inside upload image")
  try {
    const bytes = new Uint8Array(await file.arrayBuffer())
    const attachment = await invoke<Attachment>("upload_image", bytes, {
      headers: {
        "x-file-name": file.name,
        "context-type": file.type
      }
    })
    return attachment
  } catch (error) {
    notificationStore.pushError(String(error))
    return null;
  }
}

async function getChats() {
	return invoke<User[]>("get_chats");
}

const submitForm = handleSubmit(async (values) => {
  console.log("inside handlesubmt")
  var attachment: Attachment | null = null;
  if (selectedFile.value) {
      attachment = await uploadImage(selectedFile.value)
  }
  if (values.input) {
    try {
		await sendMessage(values.input, attachment);
		resetForm();
	} catch (error) {
		notificationStore.pushError(String(error));
	}
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
