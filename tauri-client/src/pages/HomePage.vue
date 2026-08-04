<template>
    <div class="bg-background">
        <AppBar/>
    </div>
    
</template>

<script setup lang="ts">
import { useState } from "../stores/state";
import AppBar from "../components/AppBar.vue";

const state = useState();
</script>