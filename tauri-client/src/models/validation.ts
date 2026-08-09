import * as z from "zod";

export const NewUser = z.object({
	username: z.string().min(3),
	password: z.string().min(3),
});

export const NewMessage = z.object({
	input: z.string().min(1).max(250),
});
