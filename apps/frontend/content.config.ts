import { defineCollection, defineContentConfig, z } from '@nuxt/content'

export default defineContentConfig({
  collections: {
    docs: defineCollection({
      type: 'page',
      source: 'docs/**/*.md',
      schema: z.object({
        title: z.string(),
        description: z.string().optional(),
        category: z.string().default('uncategorized'),
        order: z.number().optional(),
      }),
    }),

    guides: defineCollection({
      type: 'page',
      source: 'guides/**/*.md',
      schema: z.object({
        title: z.string(),
        description: z.string().optional(),
        author: z.string().default('Blueprint'),
        category: z.string().default('uncategorized'),
        thumbnail: z.string().optional(),
        order: z.number().optional(),
      }),
    }),
  },
})
