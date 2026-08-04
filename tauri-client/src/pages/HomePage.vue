<template>
    <div class="flex bg-background">
        <AppBar/>
        <main class="flex-1">
            <RouterView/>
        </main>
    </div>
    
</template>

<script setup lang="ts">
import AppBar from "../components/AppBar.vue";
</script>