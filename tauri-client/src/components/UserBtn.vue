<template>
	<button
		class="flex items-center mt-2 mr-0 w-full text-text h-8 rounded-xl opacity-70 hover:bg-primary-hover px-2.5 text-left active:bg-primary-active whitespace-nowrap"
		:class="{
		  'bg-primary-hover': user.id === chattingWith?.id
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
import { computed, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import type { User } from "../models/user";
import { useState } from "../stores/state";

const router = useRouter();
const state = useState();
const route = useRoute();

const props = defineProps<{
	user: User;
}>();

const chattingWith = ref<User | null>(null);

watch(
	() => Number(route.params.id),
	async (userId: number) => {
		const fromState = state.all_users.get(userId);

		if (!fromState) {
			chattingWith.value = await invoke<User>("get_user", {
				userId,
			});
			state.all_users.set(userId, chattingWith.value);
			return;
		}

		chattingWith.value = fromState;
	},
	{ immediate: true },
);

const unread = computed(() => {
	const chatData = state.chats_data.get(props.user.id);

	if (!chatData) return 0;
	return chatData.messages.length - chatData.read_count;
});

async function setChat(recvID: number) {
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
