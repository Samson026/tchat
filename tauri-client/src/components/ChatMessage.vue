<template>
    <div class="flex w-min rounded-2xl"
        :class="primary ? 'bg-primary' : 'bg-secondary'"
    >
        <h3 class="text-text mx-2">
            {{ state.getUsername(message.sender_id) }}:
        </h3>
        <p class="text-text mx-2 ml-1 w-fit whitespace-nowrap">
            {{ message.content }}
        </p>
    </div>
</template>

<script setup lang="ts">
import type { Message } from '../models/user';
import { useState } from '../stores/state';

const state = useState()

defineProps<{
    message: Message,
    primary: Boolean 
}>();
</script>