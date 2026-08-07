<template>
    <main class="bg-background h-screen p-10">
        <BackBtn class="absolute top-6 left-6"/>
        <div class="grid grid-cols-2 gap-3 p-20">
            <h2 class="text-text">Settings</h2>
            <input
            type="text"
            class="text-text"
            placeholder="Server Address"
            >
            </input>
            <button
            type="submit"
            class="text-text bg-primary rounded-2xl w-20 opacity-55 hover:opacity-100 hover:"
            >
                Save
            </button>
        </div>
    </main>
</template>

<script setup lang="ts">
import BackBtn from '../components/BackBtn.vue';

</script>
