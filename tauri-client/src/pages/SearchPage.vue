<template>
    <main class="h-full">
        <div class="flex flex-col bg-surface px-5 py-5 border border-border">
            <h2
                class="text-text text-xl"
            >Search</h2>
            <input 
                type="text"
                class="bg-text rounded-s"
                v-model="input"
            >
        </div>
        <div class="flex flex-col bg-surface border border-border mx-5 my-5 h-screen rounded-xl items-center">
            <div class="w-full">
                <SearchedUserBtn 
                    v-for="user in display_users"
                    :key="user.id"
                    :user="user"
                />
            </div>
        </div>
    </main>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { computed, onMounted, ref } from 'vue';
import type { User } from '../models/user';
import { useState } from '../stores/state';
import SearchedUserBtn from '../components/SearchedUserBtn.vue';

const state = useState()

const input = ref("")
const display_users = computed<User[]>(() => {
    const i = input.value.toLowerCase().trim()
    if (i === "")
        return []
    return (
        state.all_users?.filter((user) => user.username.includes(i))
        ?? []
    )
});

async function loadUsers() {
    return await invoke<User[]>("get_users")
}

onMounted(async () => {
    if (state.all_users === null) {
        state.all_users = await loadUsers()
    }
})
</script>