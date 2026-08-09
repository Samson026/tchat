<template>
	<div
		:role="type === NotifType.Error ? 'alert' : 'status'"
		class="flex w-80 items-start gap-3 rounded-xl border border-border border-l-4 bg-surface p-3 text-text shadow-xl"
		:class="{
      'border-l-error': type === NotifType.Error,
      'border-l-success': type === NotifType.Success,
      'border-l-primary': type === NotifType.Notification,
    }"
	>
		<CircleAlert
			v-if="type === NotifType.Error"
			class="mt-0.5 size-5 shrink-0 text-error"
		/>
		<CircleCheck
			v-else-if="type === NotifType.Success"
			class="mt-0.5 size-5 shrink-0 text-success"
		/>
		<Info v-else class="mt-0.5 size-5 shrink-0 text-primary" />

		<p class="min-w-0 flex-1 text-sm leading-5">{{ content }}</p>

		<button
			class="-mr-1 -mt-1 shrink-0 rounded-md p-1 text-text-muted transition-colors hover:bg-secondary-hover hover:text-text"
			type="button"
			aria-label="Dismiss notification"
			@click="destroy"
		>
			<X class="size-4" />
		</button>
	</div>
</template>

<script setup lang="ts">
import { CircleAlert, CircleCheck, Info, X } from "lucide-vue-next";
import { NotifType, useNotification } from "../stores/notifications";

const notificationsStore = useNotification();

const props = defineProps<{
	id: number;
	content: string;
	type: NotifType;
}>();

function destroy() {
	notificationsStore.notifications.splice(props.id, 1);
}
</script>
