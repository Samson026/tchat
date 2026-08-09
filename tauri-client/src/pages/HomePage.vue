<template>
	<div class="flex bg-background">
		<AppBar />
		<main class="flex-1">
			<RouterView />
		</main>
	</div>
</template>

<script setup lang="ts">
import { onBeforeMount } from "vue";
import AppBar from "../components/AppBar.vue";
import { useState } from "../stores/state";

const state = useState();

onBeforeMount(async () => {
	if (state.all_users.size === 0) {
		await state.ensureAllUsersLoaded();
	}
});
</script>
