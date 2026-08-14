interface State {
	user: User | null;
	chats_data: Map<number, ChatData>;
	all_users: Map<number, User>;
	settings: Settings | null;
}

import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import type { ChatData, Settings, User } from "../models/user";

let allUsersRequest: Promise<User[]> | null = null;

export const useState = defineStore("stateStore", {
	// arrow function recommended for full type inference
	state: (): State => {
		return {
			// all these properties will have their type inferred automatically
			user: null,
			chats_data: new Map<number, ChatData>(),
			all_users: new Map<number, User>(),
			settings: null,
		};
	},
	actions: {
		async ensureAllUsersLoaded() {
			if (this.all_users.size > 0) return;

			if (allUsersRequest === null) {
				allUsersRequest = invoke<User[]>("get_users");
			}

			try {
				const users = await allUsersRequest;

				if (this.all_users.size === 0) {
					this.all_users = new Map(users.map((user) => [user.id, user]));
				}
			} finally {
				allUsersRequest = null;
			}
		},
	},
});
