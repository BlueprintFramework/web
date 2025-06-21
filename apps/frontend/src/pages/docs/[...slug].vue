<template>
  <div v-if="data" class="space-y-4">
    <h1 class="!display !text-6xl !font-normal">
      {{ data.title || 'Untitled' }}
    </h1>
    <p v-if="data.description" class="text-default-font/60 text-lg">
      {{ data.description }}
    </p>
    <div class="prose prose-invert mt-8 max-w-none">
      <ContentRenderer :value="data" />
    </div>
  </div>
</template>

<script setup>
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

useSeoMeta({
  title: data.value?.title,
  description: data.value?.description,
})
</script>
