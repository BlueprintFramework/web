<template>
  <div v-if="data" class="space-y-8">
    <div class="space-y-4">
      <h1 class="!display truncate !text-5xl !font-normal md:!text-6xl">
        {{ data.title || 'Untitled' }}
      </h1>
      <p v-if="data.description" class="text-default-font/60 text-lg">
        {{ data.description }}
      </p>
    </div>

    <div class="prose prose-invert max-w-none">
      <ContentRenderer :value="data" class="prose-content" />
    </div>

    <div
      class="grid divide-y divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 sm:grid-cols-2 sm:divide-x sm:divide-y-0"
      v-if="prevDoc || nextDoc"
    >
      <NuxtLink
        v-if="prevDoc"
        :to="prevDoc.path"
        class="hover:text-brand-50 overflow-hidden p-4 transition-colors hover:bg-neutral-900"
      >
        <h3 class="mb-2 truncate text-lg font-medium">
          {{ prevDoc.title }}
        </h3>
        <p
          v-if="prevDoc.description"
          class="text-default-font/50 truncate text-sm"
        >
          {{ prevDoc.description }}
        </p>
      </NuxtLink>
      <div class="bg-stripes hidden h-full w-full sm:block" v-else />

      <NuxtLink
        v-if="nextDoc"
        :to="nextDoc.path"
        class="hover:text-brand-50 overflow-hidden text-nowrap p-4 transition-colors hover:bg-neutral-900"
      >
        <h3 class="mb-2 truncate text-lg font-medium">
          {{ nextDoc.title }}
        </h3>
        <p
          v-if="nextDoc.description"
          class="text-default-font/50 truncate text-sm"
        >
          {{ nextDoc.description }}
        </p>
      </NuxtLink>
      <div class="bg-stripes h-full w-full" v-else />
    </div>
  </div>
</template>

<script setup>
import { docsCategories } from '~/assets/docs.config'

definePageMeta({
  layout: 'docs',
})

const route = useRoute()

const slug = route.params.slug
const slugArray = Array.isArray(slug) ? slug : [slug]
const path = `/docs/${slugArray.join('/')}`

const { data } = await useAsyncData(`doc-${path}`, () =>
  queryCollection('docs').path(path).first()
)

if (!data.value) {
  throw createError({
    statusCode: 404,
    statusMessage: 'Page not found',
  })
}

const { data: allDocs } = await useAsyncData('all-docs-nav', () =>
  queryCollection('docs').all()
)

const { prevDoc, nextDoc } = computed(() => {
  if (!allDocs.value || !data.value) return { prevDoc: null, nextDoc: null }

  const sameCategoryDocs = allDocs.value.filter(
    (doc) => doc.category?.toLowerCase() === data.value.category?.toLowerCase()
  )

  const sortedDocs = [...sameCategoryDocs].sort((a, b) => {
    if (a.order !== undefined && b.order !== undefined) {
      return a.order - b.order
    }
    if (a.order !== undefined) return -1
    if (b.order !== undefined) return 1
    return a.title.localeCompare(b.title)
  })

  const currentIndex = sortedDocs.findIndex(
    (doc) => doc.path === data.value.path
  )

  return {
    prevDoc: currentIndex > 0 ? sortedDocs[currentIndex - 1] : null,
    nextDoc:
      currentIndex < sortedDocs.length - 1
        ? sortedDocs[currentIndex + 1]
        : null,
  }
}).value

useSeoMeta({
  title: data.value?.title,
  description: data.value?.description,
})
</script>
