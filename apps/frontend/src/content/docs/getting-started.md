---
title: Configuration
description: Learn how to configure your application
date: 2024-06-19
---

# Configuration

This guide covers the various configuration options available.

## Basic Configuration

The basic configuration is handled through the `nuxt.config.ts` file:

```typescript
export default defineNuxtConfig({
  modules: ['@nuxt/content'],
  content: {
    // Configuration options
  },
})
```

## Content Configuration

Use the `content.config.ts` file to define your collections:

```typescript
import { defineCollection, defineContentConfig, z } from '@nuxt/content'

export default defineContentConfig({
  collections: {
    docs: defineCollection({
      type: 'page',
      source: 'content/docs/**/*.md',
    }),
  },
})
```

## Environment Variables

You can use environment variables for sensitive configuration:

- `DATABASE_URL` - Database connection string
- `API_KEY` - External API key
- `SECRET_KEY` - Application secret

## Advanced Options

For advanced use cases, you can customize:

- Custom transformers
- Plugin configuration
- Build optimizations

See the [API reference](/docs/api) for detailed options.
