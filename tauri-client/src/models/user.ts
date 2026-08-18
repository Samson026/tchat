export interface User {
	id: number;
	username: string;
}

export interface Chat {
	id: number;
	username: string;
	user_id: number;
	read_count: number;
}

export interface ChatId {
	id: number;
	user_1_id: number;
	user_2_id: number;
}

export interface ChatData {
	user: User;
	id: number | null;
	messages: Message[];
	read_count: number;
}

export interface Message {
	chat_id: number | null;
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
