interface State {
	user: User | null;
	chats_data: Map<number, User>;
	messages: Message[] | null;
	chating_with: User | null;
	all_users: User[] | null;
	settings: Settings | null;
}

import { defineStore } from "pinia";
import type { Message, Settings, User } from "../models/user";
export const useState = defineStore("stateStore", {
	// arrow function recommended for full type inference
	state: (): State => {
		return {
			// all these properties will have their type inferred automatically
			user: null,
			chats_data: new Map<number, User>,
			messages: null,
			chating_with: null,
			all_users: null,
			settings: null,
		};
	},
  actions: {
    addNotification(userId: number) {
      const user = this.chats_data.get(userId)

      if (user)
        user.unread += 1
    }
  },
});
