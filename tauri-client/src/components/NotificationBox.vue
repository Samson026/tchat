<template>
    <div role="status"
    class="w-64 rounded-xl border text-text p-2 flex shadow m-1"
    :class="{
      'bg-error': type === NotifType.Error,
      'bg-success': type === NotifType.Success,
      'bg-primary': type === NotifType.Notification
    }"
    >
    <p>{{ content }}</p>
    <button
    class="ml-auto"
    type="button"
    @click="destory"
    >
        <X/>
    </button>
    </div>
</template>

<script setup lang="ts">
import { X } from 'lucide-vue-next';
import { NotifType, useNotification } from '../stores/notifications';

const notificationsStore = useNotification();

const props = defineProps<{
  id: number
  content: string
  type: NotifType
}>()

function destory() {
    notificationsStore.notifications.splice(props.id, 1)
}
</script>
