<template>
    <button 
        class="mt-2 mr-0 block w-full text-text h-8 rounded-xl opacity-70 hover:bg-primary-hover px-2.5 text-left active:bg-primary-active"
        type="button"
    >
    {{ user.username }}
    </button>
</template>

<script setup lang="ts">
import type { User } from '../models/user';
defineProps<{
    user: User
}>();
</script>