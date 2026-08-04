<template>
    <main>
        <div>
            <ChatMessage
                v-for="(message, index) in state.chat"
                :key="index"
                :message="message"
                :primary="message.sender_id === state.user?.id"
            />
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
        sender_id: state.user?.id,
        receiver_id: recv_id
    })
}

onMounted(async () => {
    if (state.user)
        await getChat(state.user.id)
})
</script>