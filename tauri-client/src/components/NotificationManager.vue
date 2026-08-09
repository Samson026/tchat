<template>
	<div class="fixed left-1/2 top-6 z-50 flex -translate-x-1/2 flex-col gap-3">
		<NotificationBox
			v-for="(notif, index) in notificationStore.notifications"
			:key="index"
			:id="index"
			:type="notif.type"
			:content="notif.content"
		/>
	</div>
</template>

<script setup lang="ts">
import { useNotification } from "../stores/notifications";
import NotificationBox from "./NotificationBox.vue";

const notificationStore = useNotification();
</script>
