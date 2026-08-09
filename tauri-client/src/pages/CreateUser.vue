<template>
	<main
		class="relative grid min-h-screen place-items-center bg-background p-6 text-text"
	>
		<BackBtn class="absolute left-6 top-6" />

		<h1 class="text-center text-5xl font-semibold">tChat</h1>
		<div class="min-h-64 flex w-full max-w-sm rounded-xl bg-surface shadow-2xl">
			<div class="flex w-full flex-col px-20 py-4">
				<h1
					class="whitespace-nowrap text-text text-center text-2xl font-semibold opacity-50 my-4 px-10"
				>
					Create Account
				</h1>

				<form class="flex flex-col h-max" @submit="submitForm">
					<input
						type="text"
						class="w-full max-w-sm rounded-lg border border-border text-text bg-grey my-1 font-extralight"
						placeholder="Username"
						v-model="username"
					>
					<p v-if="errors.username" class="text-error text-sm">
						{{ errors.username }}
					</p>
					<input
						type="password"
						class="w-full max-w-sm rounded-lg border border-border text-text bg-grey my-1 font-extralight"
						placeholder="Password"
						v-model="password"
					>
					<p v-if="errors.password" class="text-error text-sm">
						{{ errors.password }}
					</p>
					<button
						class="bg-primary w-30 self-center rounded-lg hover:bg-primary-hover my-2"
						type="submit"
					>
						Create
					</button>
				</form>
			</div>
		</div>
	</main>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { ref } from "vue";
import { useRouter } from "vue-router";
import BackBtn from "../components/BackBtn.vue";
import type { User } from "../models/user";
import { NewUser } from "../models/validation";
import { useState } from "../stores/state";

const user = ref<User | null>(null);
const state = useState();
const router = useRouter();

const { defineField, handleSubmit, errors } = useForm({
	validationSchema: toTypedSchema(NewUser),
});

const [username] = defineField("username");
const [password] = defineField("password");

const submitForm = handleSubmit(async (values) => {
	user.value = await invoke<User>("create_user", {
		username: values.username,
		password: values.password,
	});

	if (user.value.username) {
		state.user = user.value;
		router.push("/home");
	}
});
</script>
