import { defineCollection, z } from 'astro:content';

const blog = defineCollection({
  schema: z.object({
    title: z.string(),
    description: z.string(),
    created: z
      .string()
      .or(z.date())
      .transform((val) => new Date(val)),
    updated: z
      .string()
      .optional()
      .transform((str) => (str ? new Date(str) : undefined)),
    image: z.string().optional(),
  }),
});

export const collections = { blog };
