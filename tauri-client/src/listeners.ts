import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { ChatData, Message, User } from "./models/user";
import { IncomingWsMessage } from "./models/validation";
import { useNotification } from "./stores/notifications";
import { useState } from "./stores/state";

export async function setupListeners() {
	const state = useState();
	const notificationStore = useNotification();

	listen<string>("ws-message", async (event) => {
		let payload: unknown;

		try {
			payload = JSON.parse(event.payload);
		} catch {
			notificationStore.pushError("Invalid websocket message payload");
			return;
		}

		const parsed = IncomingWsMessage.safeParse(payload);

		if (!parsed.success) {
			notificationStore.pushError("Invalid websocket message payload");
			return;
		}

		const message: Message = parsed.data;
		console.log(`got message ${event.payload}`);
		// currently chatting with user
		if (state.chats_data.get(message.sender_id)) {
			state.chats_data.get(message.sender_id)?.messages.push(message);
			return;
		}
		console.log(message);
		let user = state.all_users.get(message.sender_id);

		if (!user) {
			user = await invoke<User>("get_user", {
				userId: message.sender_id,
			});
			state.all_users.set(user.id, user);
		}

		const chatData = {
			user,
			id: message.chat_id,
			messages: [message],
		} as ChatData;

		state.chats_data.set(message.sender_id, chatData);
	});

	listen<string>("ws-error", async (event) => {
		notificationStore.pushError(event.payload);
	});
}
