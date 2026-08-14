import { listen } from "@tauri-apps/api/event";
import type { ChatData, Message } from "./models/user";
import { useNotification } from "./stores/notifications";
import { useState } from "./stores/state";

export function setupListeners() {
	const state = useState();
	const notificationStore = useNotification();

	listen<string>("ws-message", (event) => {
		const message = JSON.parse(event.payload) as Message;
		console.log(`got message ${event.payload}`);
		// currently chatting with user
		if (state.chats_data.get(message.sender_id)) {
			state.chats_data.get(message.sender_id)?.messages.push(message);
			return;
		}

		// Pull user from db, get rid of local all users
		const user = state.all_users.get(message.sender_id);
		if (user) {
			console.log(`adding user ${user}`);

			const chatData = {
				user,
				id: null,
				messages: [message],
			} as ChatData;

			state.chats_data.set(message.sender_id, chatData);
		}
	});

	listen<string>("ws-error", async (event) => {
		notificationStore.pushError(event.payload);

		// await invoke("connect_ws");
	});
}
