<template>
  <div v-if="data" class="space-y-8">
    <div class="space-y-3">
      <h1 class="!text-4xl">
        {{ data.title || 'Untitled' }}
      </h1>
      <div
        class="inline-block rounded-full border border-neutral-700 bg-neutral-900"
      >
        <div
          class="flex flex-row items-center divide-x divide-neutral-700 px-0.5"
        >
          <div class="flex flex-row items-center gap-1.5 px-1.5 py-0.5">
            <Icon name="memory:clock" mode="svg" />
            <span>
              Updated <NuxtTime :datetime="data?.updated" :relative="true" />
            </span>
          </div>
          <span class="px-1.5 py-0.5">
            <NuxtTime :datetime="data?.updated" />
          </span>
        </div>
      </div>
    </div>

    <div class="max-w-none">
      <ContentRenderer :value="data" class="prose-content" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { docsCategories, defaultCategory } from '~/assets/docs.config'

const route = useRoute()

const slug = route.params.slug
const path = `/legal/${route.params.slug}`

const { data } = await useAsyncData(`legal-${path}`, () =>
  queryCollection('legal').path(path).first()
)

if (!data.value) {
  throw createError({
    statusCode: 404,
    statusMessage: 'Page not found',
  })
}

useSeoMeta({
  title: data.value?.title,
  description: data.value?.description,
})
</script>
