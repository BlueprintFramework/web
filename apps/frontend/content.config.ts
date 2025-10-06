import { defineCollection, defineContentConfig, z } from '@nuxt/content'

export default defineContentConfig({
  collections: {
    // Documentation documents
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

    // Guide articles
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
        unlisted: z.boolean().optional().default(false),
      }),
    }),

    // Legal documents
    legal: defineCollection({
      type: 'page',
      source: 'legal/*.md',
      schema: z.object({
        title: z.string(),
        updated: z.string(),
      }),
    }),
  },
})
