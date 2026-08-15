<template>
	<main class="flex flex-col h-screen">
		<div clas="px-10 py-10"></div>
		<h2 class="text-text m-5">Chatting with: {{ chattingWith.username }}</h2>
		<div class="flex min-h-0 flex-1 flex-col justify-end">
			<div
				class="flex flex-col col h-full justify-end px-10 py-10 overflow-y-auto"
			>
				<ChatMessage
					v-for="(message, index) in state.chats_data.get(chattingWith.id)?.messages"
					:key="index"
					:message="message"
					:primary="message.sender_id === state.user?.id"
					:class="message.sender_id === state.user?.id ? 'self-end' : 'self-start'"
				/>
			</div>
			<div
				class="bg-surface w-full h-30 border border-border rounded-xl px-2 py-2 flex place-items-center"
			>
				<div v-if="imagePreview" class="relative m-2 h-10 w-10 shrink-0">
					<img
						:src="imagePreview"
						alt="Attachment preview"
						class="h-full w-full rounded object-cover"
					>
					<button
						type="button"
						aria-label="Remove attachment"
						class="absolute -right-2 -top-2 flex h-5 w-5 items-center justify-center rounded-full bg-error text-white"
						@click="removeAttach"
					>
						<X class="h-3 w-3" />
					</button>
				</div>

				<form
					class="flex h-full w-full items-center gap-2"
					@submit="submitForm"
				>
					<label
						class="flex h-full w-14 shrink-0 cursor-pointer items-center justify-center rounded-2xl bg-secondary"
					>
						<span class="sr-only">Upload image</span>
						<CameraIcon class="h-6 w-6 text-text" />
						<input
							type="file"
							class="hidden"
							accept="image/*"
							@change="onImageSelected"
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
import { CameraIcon, X } from "lucide-vue-next";
import { useForm } from "vee-validate";
import { computed, ref, watch } from "vue";
import { useRoute } from "vue-router";
import ChatMessage from "../components/ChatMessage.vue";
import { ChatId, type Attachment, type Message, type User } from "../models/user.ts";
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

const selectedFile = ref<File | null>(null);
const imagePreview = ref<string | null>(null);

const chattingWith = computed<User>(() => {
	const user = state.all_users.get(Number(route.params.id));

	if (!user)
		return {
			id: 0,
			username: "not found",
		} as User;

	return user;
});

// Watch message count to update read status
watch(
	() => {
		const chatId = chattingWith.value.id;
		return {
			chatId,
			messageCount: state.chats_data.get(chatId)?.messages.length ?? 0,
		};
	},
	async ({ chatId, messageCount }) => {
		const chatData = state.chats_data.get(chatId);

		if (!chatData) return;
		chatData.read_count = messageCount;

		await invoke("update_read", {
			chatId: chatData.id,
			readCount: messageCount,
		});
	},
);

function removeAttach() {
	selectedFile.value = null;
	imagePreview.value = null;
}

function onImageSelected(event: Event) {
	console.log("inside on image select");
	const input = event.target as HTMLInputElement;
	selectedFile.value = input.files?.[0] ?? null;
	if (selectedFile.value === null) return;

	if (imagePreview.value) {
		URL.revokeObjectURL(imagePreview.value);
	}

	imagePreview.value = URL.createObjectURL(selectedFile.value);
}

async function sendMessage(message: string, attachment: Attachment | null) {
	if (!state.user) return;

	const chatData = state.chats_data.get(chattingWith.value.id);

	const msg: Message = {
		sender_id: state.user.id,
		recv_id: chattingWith.value.id,
		content: message,
		attachment: attachment?.id ?? null,
	};

	if (!chatData) {
		notificationStore.pushError("Error: No chat data")
		return
	}

	await invoke("send", {
		message: msg,
	});
	
	// check if new chat
	if (!chatData?.id) {
		const chatIds = await invoke<ChatId>("get_chat_by_ids", {
			receiverId: chattingWith.value.id
		})

		chatData.id = chatIds.id
	}

	// add msg to local data
	chatData.messages.push(msg);
}

async function uploadImage(file: File) {
	console.log("inside upload image");
	try {
		const bytes = new Uint8Array(await file.arrayBuffer());
		const attachment = await invoke<Attachment>("upload_image", bytes, {
			headers: {
				"x-file-name": file.name,
				"context-type": file.type,
			},
		});
		return attachment;
	} catch (error) {
		notificationStore.pushError(String(error));
		return null;
	}
}

const submitForm = handleSubmit(async (values) => {
	console.log("inside handlesubmt");
	var attachment: Attachment | null = null;
	if (selectedFile.value) {
		attachment = await uploadImage(selectedFile.value);
	}
	if (values.input) {
		try {
			await sendMessage(values.input, attachment);
			resetForm();
			selectedFile.value = null;
			imagePreview.value = null;
		} catch (error) {
			notificationStore.pushError(String(error));
		}
	}
});
</script>
