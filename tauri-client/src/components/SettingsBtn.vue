<template>
    <div>
        <button
        type="button"
        class="grid place-items-center items-center hover:bg-primary-hover active:bg-primary-active rounded-full w-10 h-10"
        @click="openSettings()"
        >
            <SettingsIcon class="self-center"/>
        </button>
    </div>
</template>

<script setup lang="ts">
import { SettingsIcon } from 'lucide-vue-next';
import { useRouter } from 'vue-router';

const router = useRouter()

function openSettings() {
  router.push("/settings")
}
</script>
