import * as z from "zod";

export const NewUser = z.object({
	username: z.string().min(3),
	password: z.string().min(3),
});

export const NewMessage = z.object({
	input: z.string().min(1).max(250).optional(),
	image: z.instanceof(File).optional(),
});

export const IncomingWsMessage = z.object({
	chat_id: z.number().int(),
	sender_id: z.number().int(),
	recv_id: z.number().int(),
	content: z.string(),
	attachment: z.string().nullable(),
});
