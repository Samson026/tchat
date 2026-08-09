import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Message } from "./models/user";
import { useNotification } from "./stores/notifications";
import { useState } from "./stores/state";

export function setupListeners() {
	const state = useState();
	const notificationStore = useNotification();

	listen<string>("ws-message", (event) => {
		const message = JSON.parse(event.payload) as Message;
		console.log(`got message ${event.payload}`);
		if (state.chating_with?.id === message.sender_id) {
			if (state.messages === null) {
				state.messages = [message];
			}
			state.messages.push(message);
			return;
		}
		if (state.chats_data.has(message.sender_id)) {
			state.addNotification(message.sender_id);
			return;
		}
		const user = state.all_users.get(message.sender_id);
		console.log(user);
		if (user) {
			console.log(`adding user ${user}`);
			user.unread = 1;
			state.chats_data.set(message.sender_id, user);
		}
	});

	listen<string>("ws-disconnected", async () => {
		await invoke("connect_ws");
	});

	listen<string>("ws-error", async (error) => {
		notificationStore.pushError(String(error));

		await invoke("connect_ws");
	});
}
