export interface User {
	id: number;
	username: string;
}

export interface Chat {
	id: number;
	username: string,
	user_id: number,
	last_read_id: number
}

export interface ChatData {
	user: User,
	id: number
	messages: Message[],
	unread: number
}

export interface Message {
	sender_id: number;
	recv_id: number;
	content: string;
	attachment: string | null;
}

export interface Settings {
	server_address: string;
}

export interface Attachment {
	id: string;
}
