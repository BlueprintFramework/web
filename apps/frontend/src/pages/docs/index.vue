<template>
  <div class="space-y-4">
    <h1 class="!display !text-6xl !font-normal">Documentation</h1>
    <p class="text-default-font/60 text-lg">
      Documentation purpose-built for system administrators and extension
      developers
    </p>
  </div>

  <div
    class="grid divide-y divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
  >
    <div
      v-for="(categoryData, category) in categories"
      :key="category"
      class="flex items-center justify-between p-4"
    >
      <div class="flex items-center gap-3">
        <Icon :name="categoryData.icon" :size="32" />
        <h2 class="text-2xl font-semibold">{{ category }}</h2>
      </div>
      <p class="opacity-75">
        {{ categoryData.docCount }}
        {{ categoryData.docCount === 1 ? 'document' : 'documents' }}
      </p>
    </div>
  </div>

  <NuxtImg
    src="/img/documentation.jpeg"
    class="w-full rounded-3xl border border-neutral-700 bg-neutral-950 object-cover"
    :height="600"
    :width="1280"
  />
</template>

<script setup lang="ts">
import { docsCategories, defaultCategory } from '~/assets/docs.config'

definePageMeta({
  layout: 'docs',
})

const { data: docs } = await useAsyncData('docs-index', () => {
  return queryCollection('docs').all()
})

const categories = computed(() => {
  if (!docs.value) return {}

  const grouped = docs.value.reduce(
    (acc, doc) => {
      const categoryKey = doc.category?.toLowerCase() || 'general'
      const categoryConfig = docsCategories[categoryKey] || defaultCategory
      const categoryLabel = categoryConfig.label || doc.category || 'General'

      if (!acc[categoryLabel]) {
        acc[categoryLabel] = {
          icon: categoryConfig.icon,
          order: categoryConfig.order || 999,
          docCount: 0,
          firstDoc: doc._path,
        }
      }
      acc[categoryLabel].docCount++
      // Keep the first doc path (assuming they're sorted)
      if (
        doc.order &&
        (!acc[categoryLabel].docOrder ||
          doc.order < acc[categoryLabel].docOrder)
      ) {
        acc[categoryLabel].firstDoc = doc._path
        acc[categoryLabel].docOrder = doc.order
      }
      return acc
    },
    {} as Record<
      string,
      {
        icon: string
        order: number
        docCount: number
        firstDoc: string
        docOrder?: number
      }
    >
  )

  // Sort categories by their defined order
  const sortedGrouped = Object.entries(grouped)
    .sort(([, a], [, b]) => a.order - b.order)
    .reduce(
      (acc, [key, value]) => {
        acc[key] = value
        return acc
      },
      {} as typeof grouped
    )

  return sortedGrouped
})
</script>
