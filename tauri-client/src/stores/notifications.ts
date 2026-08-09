import { defineStore } from "pinia";
import { NOTIFICATION_PREFIXES } from "../constants";

export enum NotifType {
	Error,
	Success,
	Notification,
}

interface Notification {
	content: string;
	type: NotifType;
}

interface State {
	notifications: Notification[];
}

export const useNotification = defineStore("notificationStore", {
	state: (): State => {
		return {
			notifications: [],
		};
	},
	actions: {
		pushError(content: string) {
			this.notifications.push({
				content: NOTIFICATION_PREFIXES.ERROR + content,
				type: NotifType.Error,
			});
		},
		pushSuccess(content: string) {
			this.notifications.push({
				content: NOTIFICATION_PREFIXES.SUCCESS + content,
				type: NotifType.Success,
			});
		},
		pushNotification(content: string) {
			this.notifications.push({
				content: NOTIFICATION_PREFIXES.NOTIFICATION + content,
				type: NotifType.Notification,
			});
		},
	},
});
