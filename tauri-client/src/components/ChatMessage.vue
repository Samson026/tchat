<template>
    <div class="flex w-min rounded-2xl my-0.5"
        :class="primary ? 'bg-primary' : 'bg-secondary'"
    >
        <p class="text-text px-3 ml-1 w-fit whitespace-nowrap min-w-5">
            {{ message.content }}
        </p>
    </div>
</template>

<script setup lang="ts">
import type { Message } from '../models/user';

defineProps<{
    message: Message,
    primary: boolean 
}>();
</script>