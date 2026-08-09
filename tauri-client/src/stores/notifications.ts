export enum NotifType {
  Error,
  Success,
  Notification
}

interface Notification {
  content: string,
  type: NotifType
}

interface State {
  notifications: Notification[];
}

import { defineStore } from "pinia";

export const useNotification = defineStore("notificationStore", {
  state: (): State => {
    return {
      notifications: [],
    }
  }
})
