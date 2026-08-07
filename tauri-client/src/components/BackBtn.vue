<template>
    <button
    type="button"
    class="grid place-items-center items-center hover:bg-primary-hover active:bg-primary-active rounded-full w-10 h-10"
    @click="goBack()"
    >
        <ChevronLeft class="self-center text-text"/>
    </button>
</template>

<script setup lang="ts">
import { ChevronLeft } from 'lucide-vue-next';
import { useRouter } from 'vue-router';

const router = useRouter()

function goBack() {
  router.back()
}
</script>
