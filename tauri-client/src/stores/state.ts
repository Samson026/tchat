interface State {
	user: User | null;
	chats_id: Set<Number>;
	chats_data: User[];
	messages: Message[] | null;
	chating_with: User | null;
	all_users: User[] | null;
}

import { defineStore } from "pinia";
import type { Message, User } from "../models/user";
export const useState = defineStore("stateStore", {
	// arrow function recommended for full type inference
	state: (): State => {
		return {
			// all these properties will have their type inferred automatically
			user: null,
			chats_id: new Set<Number>,
			chats_data: [],
			messages: null,
			chating_with: null,
			all_users: null
		};
	},
	getters: {
		getUsername: (state) => {
			return (userId: number): string => {
				const user = state.chats_data.find((user) => user.id === userId);
				return user?.username ?? "Unknown user";
			};
		},
	},
});
