<template>
    <main class="h-full">
        <div clas="px-10 py-10">

        </div>
        <div class="flex flex-col h-full justify-end">
            <div class="flex flex-col col h-full justify-end px-10 py-10">
                <ChatMessage
                    v-for="(message, index) in state.chat"
                    :key="index"
                    :message="message"
                    :primary="message.sender_id === state.user?.id"
                    :class="message.sender_id === state.user?.id ? 'self-end' : 'self-start'"
                />
            </div>
            <div class="bg-surface w-full h-20 border border-border rounded-xl px-2 py-2">
                <form class="flex h-full items-center">
                    <input
                        type="text"
                        placeholder="Message"
                        class="text-text w-full h-full mx-2"
                    >
                    <button class="text-text bg-primary rounded-2xl min-w-15"
                        type="submit"
                    >
                        Send
                    </button>
                </form>
            </div>
            
        </div>
    </main>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import ChatMessage from '../components/ChatMessage.vue';
import type { Message } from '../models/user.ts';
import { useRoute } from 'vue-router';
import { onMounted } from 'vue';
import { useState } from '../stores/state.ts';

const route = useRoute()
const state = useState()

async function getChat(userID: number) {
    const recv_id: number = Number(route.params.id)
    state.chat = await invoke<Message[]>("get_messages", {
        senderId: userID,
        receiverId: recv_id
    })
}

onMounted(async () => {
    if (state.user)
        await getChat(state.user.id)
})
</script>