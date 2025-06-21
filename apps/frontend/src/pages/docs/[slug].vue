<template>
  <div class="space-y-4">
    <h1 class="!display !text-6xl !font-normal">
      {{ post.title || 'Untitled' }}
    </h1>
    <p class="text-default-font/60 text-lg">
      {{ post.description || 'No description provided' }}
    </p>
  </div>
  <div>
    <ContentRenderer :value="post" />
  </div>
</template>

<script setup>
definePageMeta({
  layout: 'docs',
})

const slug = useRoute().params.slug
const { data: post } = await useAsyncData(`docs-${slug}`, () => {
  return queryCollection('docs').path(`/docs/${slug}`).first()
})

useSeoMeta({
  title: post.value?.title,
  description: post.value?.description,
  ogTitle: post.value?.title,
  ogDescription: post.value?.description,
})
</script>
