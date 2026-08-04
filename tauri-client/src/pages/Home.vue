<template>
    <h1>Welcome {{ state.$state.user?.username }}</h1>
</template>

<script setup lang="ts">
    import { useState } from '../stores/state';

    const state = useState()
</script>