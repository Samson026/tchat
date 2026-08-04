import { listen } from "@tauri-apps/api/event";
import type { Message } from "./models/user";
import { useState } from "./stores/state";

export function setupListeners() {
	const state = useState();

	listen<string>("ws-message", (event) => {
		const message = JSON.parse(event.payload) as Message;
		console.log(`got message ${event.payload}`);
		if (state.chating_with?.id === message.sender_id) {
			if (state.chat === null) {
				state.chat = [message];
			}
			state.chat.push(message);
		}
	});
}
