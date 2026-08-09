<template>
    <NotificationManager />
	<RouterView />
</template>

<script setup lang="ts">
import NotificationManager from './components/NotificationManager.vue';
import { NotifType, useNotification } from './stores/notifications';
const ns = useNotification()
NotifType
ns.notifications.push({
  content: "hi",
  type: NotifType.Success
})
ns.notifications.push({
  content: "hi",
  type: NotifType.Notification
})
ns.notifications.push({
  content: "hi",
  type: NotifType.Error
})
</script>
