<template>
	<button
		class="flex items-center mt-2 mr-0 w-full text-text h-8 rounded-xl opacity-70 hover:bg-primary-hover px-2.5 text-left active:bg-primary-active whitespace-nowrap"
		:class="{
		  'bg-primary-hover': user.id === state.chating_with?.id
  		}"
		type="button"
		@click="setChat(user.id)"
	>
		<span>{{ user.username }}</span>
		<span v-if="unread > 0" class="ml-auto">
			{{ unread }}
		</span>
	</button>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed } from "vue";
import { useRouter } from "vue-router";
import type { User } from "../models/user";
import { useState } from "../stores/state";

const router = useRouter();
const state = useState();

const props = defineProps<{
	user: User;
}>();

const unread = computed(() => {
	const chatData = state.chats_data.get(props.user.id);

	if (!chatData) return 0;
	console.log(chatData);
	console.log(chatData.messages.length);
	console.log(`unread ${chatData.read_count - chatData.messages.length}`);
	return chatData.messages.length - chatData.read_count;
});

async function setChat(recvID: number) {
	state.chating_with = props.user;
	const chatData = state.chats_data.get(recvID);
	if (chatData) {
		// update last read message
		await invoke("update_read", {
			chatId: chatData.id,
			readCount: chatData.messages.length,
		});

		// update local data
		chatData.read_count = chatData.messages.length;
	}

	router.push(`/home/chat/${recvID}`);
}
</script>
