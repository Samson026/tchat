<template>
	<button
		class="mt-2 mr-0 block w-full text-text h-8 rounded-xl opacity-70 hover:bg-primary-hover px-2.5 text-left active:bg-primary-active"
		type="button"
		@click="setChat(user.id)"
	>
		{{ user.username }}
	</button>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import type { Message, User } from "../models/user";
import { useState } from "../stores/state";

const router = useRouter();
const state = useState();

const props = defineProps<{
	user: User;
}>();

async function setChat(recvID: number) {
	state.chating_with = props.user;
	state.messages = await getChat(recvID);

	router.push(`/home/chat/${recvID}`);
}

async function getChat(recvID: number) {
	return await invoke<Message[]>("get_messages", {
		receiverId: recvID,
	});
}
</script>
