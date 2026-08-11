<template>
	<div class="flex flex-col w-full">
		<div
			class="my-0.5 w-fit max-w-[40%] rounded-2xl self-end"
			:class="primary ? 'bg-primary' : 'bg-secondary'"
		>
			<p
				class="whitespace-pre-wrap break-word text-text px-3 py-2 text-left w-fit"
			>
				{{ message.content }}
			</p>
		</div>
		<div v-if="imageUrl" class="max-w-80 self-end mt-1">
			<img :src="imageUrl" alt="" class="rounded">
		</div>
	</div>
</template>

<script setup lang="ts">
import { path } from "@tauri-apps/api";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import type { Message } from "../models/user";
import { useNotification } from "../stores/notifications";

const notificationStore = useNotification();

const imageUrl = ref<string | null>(null);

const props = defineProps<{
	message: Message;
	primary: boolean;
}>();

onMounted(async () => {
	if (props.message.attachment) {
		try {
			await invoke("download_image", {
				fileId: props.message.attachment,
			});

			const dataDir = await path.appDataDir();
			imageUrl.value = convertFileSrc(
				await path.join(dataDir, "attachments", props.message.attachment),
			);
		} catch (error) {
			notificationStore.pushError(String(error));
		}
	}
});
</script>
