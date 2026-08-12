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
			{{ state.chats_data.get(user.id)?.unread }}
		</span>
	</button>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed } from "vue";
import { useRouter } from "vue-router";
import type { Message, User } from "../models/user";
import { useState } from "../stores/state";

const router = useRouter();
const state = useState();

const props = defineProps<{
	user: User;
}>();

const unread = 0;

async function setChat(recvID: number) {
	state.chating_with = props.user;
	const chatData = state.chats_data.get(recvID);
	if (chatData) chatData.unread = 0;

	router.push(`/home/chat/${recvID}`);
}
</script>
