interface State {
	user: User | null;
	users: User[] | null;
	chat: Message[] | null;
	chating_with: User | null;
}

import { defineStore } from "pinia";
import type { Message, User } from "../models/user";
export const useState = defineStore("stateStore", {
	// arrow function recommended for full type inference
	state: (): State => {
		return {
			// all these properties will have their type inferred automatically
			user: null,
			users: null,
			chat: null,
			chating_with: null
		};
	},
	getters: {
		getUsername: (state) => {
			return (userId: number): string => {
				const user = state.users?.find((user) => user.id === userId);
				return user?.username ?? "Unknown user";
			};
		},
	},
});
