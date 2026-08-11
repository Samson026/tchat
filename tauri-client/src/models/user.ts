export interface User {
	id: number;
	username: string;
	unread: number;
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
