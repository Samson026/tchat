<template>
    <main>
        <button 
            class="hover:bg-primary-hover h-10 flex justify-center rounded-xl m-1 w-full"
            @click="chatUser(user)"
        >
            <p class="text-text self-center">{{ user.username }}</p>
        </button>
    </main>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { User } from '../models/user';
import { useState } from '../stores/state';

const state = useState()
const router = useRouter()

function chatUser(user: User) {
    if (state.chats === null) {
        state.chats = [user]
        router.push(`/home/chat/${user.id}`)
        return
    }
    state.chats.push(user)

    router.push(`/home/chat/${user.id}`)
}

defineProps<{
	user: User;
}>();
</script>