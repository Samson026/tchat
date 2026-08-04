<template>
    <button 
        class="mt-2 mr-0 block w-full text-text h-8 rounded-xl opacity-70 hover:bg-primary-hover px-2.5 text-left active:bg-primary-active"
        type="button"
        @click="setChat(user.id)"
    >
    {{ user.username }}
    </button>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import type { User, Message } from '../models/user';
import { useState } from '../stores/state';
import { invoke } from '@tauri-apps/api/core';

const router = useRouter()
const state = useState()


defineProps<{
    user: User
}>();

async function setChat(recvID: number) {
    if (state.user === null) {
        console.log("Error user not logged in")
        return
    }

    await getChat(state.user.id, recvID)

    router.push(`/home/${recvID}`)
}

async function getChat(userID: number, recvID: number) {
    state.chat = await invoke<Message[]>("get_messages", {
        senderId: userID,
        receiverId: recvID
    })
}
</script>