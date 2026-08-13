export interface User {
	id: number;
	username: string;
}

export interface Chat {
	id: number;
	username: string,
	user_id: number,
	read_count: number
}

export interface ChatData {
	user: User,
	id: number | null
	messages: Message[],
	read_count: number
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
