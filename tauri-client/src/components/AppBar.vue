<template>
   <div class="bg-surface w-43 h-screen rounded-lg px-4 py-5 border border-border">
        <h1 class="text-text font-semibold text-3xl tracking-tight">
            tChat
        </h1>
        <div class="flex flex-col">
            <p class="text-text opacity-50 mt-10 text-xl">Chats:</p>
            <UserBtn
                v-for="user in state.users"
                :key="user.id"
                :user="user"
                class="-ml-2"
            />
        </div>
   </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted } from 'vue';
import type { User } from '../models/user.ts';
import { useState } from '../stores/state.ts';
import UserBtn from './UserBtn.vue';

const state = useState()

onMounted(async () => {
    state.users = await invoke<User[]>("get_users", {});
})

</script>