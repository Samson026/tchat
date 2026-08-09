import * as z from "zod";

export const NewUser = z.object({
  username: z.string().min(3),
  password: z.string().min(3)
})