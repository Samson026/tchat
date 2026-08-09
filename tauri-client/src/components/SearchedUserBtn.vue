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
import type { User } from "../models/user";
import { useState } from "../stores/state";

const state = useState();
const router = useRouter();

function chatUser(user: User) {
	state.chats_data.set(user.id, user);

	router.push(`/home/chat/${user.id}`);
}

defineProps<{
	user: User;
}>();
</script>
