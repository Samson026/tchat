<template>
	<div
		class="my-0.5 w-fit max-w-[40%] rounded-2xl"
		:class="primary ? 'bg-primary' : 'bg-secondary'"
	>
		<p
			class="whitespace-pre-wrap break-word text-text px-3 py-2 text-left w-fit"
		>
			{{ message.content }}
		</p>
	</div>
</template>

<script setup lang="ts">
import type { Message } from "../models/user";

defineProps<{
	message: Message;
	primary: boolean;
}>();
</script>
