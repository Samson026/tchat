export interface User {
	id: number;
	username: string;
}

export interface Message {
	sender_id: number;
	recv_id: number;
	content: string;
}
