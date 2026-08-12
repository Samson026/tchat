<template>
	<main>
		<button
			class="hover:bg-primary-hover h-10 flex justify-center rounded-xl m-1 w-full"
			@click="chatUser(user)"
			type="button"
		>
			<p class="text-text self-center">{{ user.username }}</p>
		</button>
	</main>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import type { ChatData, User } from "../models/user";
import { useState } from "../stores/state";

const state = useState();
const router = useRouter();

function chatUser(user: User) {
	state.chating_with = user;

	const chatData: ChatData = {
		user: user,
		id: null,
		messages: [],
		unread: 0
	}
	
	state.chats_data.set(user.id, chatData);

	router.push(`/home/chat/${user.id}`);
}

defineProps<{
	user: User;
}>();
</script>
