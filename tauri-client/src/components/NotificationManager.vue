<template>
    <div class="flex flex-col absolute top-6 z-50 left-1/2 -translate-x-1/2 min-h-20">
        <NotificationBox v-for="(notif, index) in notificationStore.notifications"
        :key=index
        :id=index
        :type="notif.type"
        :content="notif.content"
        />
    </div>
</template>

<script setup lang="ts">
import { useNotification } from '../stores/notifications';
import NotificationBox from './NotificationBox.vue';

const notificationStore = useNotification()
</script>
