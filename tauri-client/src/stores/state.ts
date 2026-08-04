interface State {
	user: User | null;
	users: User[] | null;
}

import { defineStore } from "pinia";
import type { User } from "../models/user";
export const useState = defineStore("stateStore", {
	// arrow function recommended for full type inference
	state: (): State => {
		return {
			// all these properties will have their type inferred automatically
			user: null,
			users: null,
		};
	},
});
