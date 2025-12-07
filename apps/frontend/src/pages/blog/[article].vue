<template>
  <template v-if="post">
    <NuxtImg
      :src="`/img/blog/thumbnails/${post.thumbnail || 'default.jpeg'}`"
      :height="766"
      class="md:h-50 h-35 w-full border-b border-neutral-700 object-cover"
    />

    <div class="space-y-2 border-b border-neutral-700 p-4 md:text-center">
      <p class="md:text-3xl! text-xl font-bold">
        {{ post.title || 'Untitled post' }}
      </p>
      <p>{{ post.description || 'No description provided' }}</p>
      <p class="text-default-font/60">
        <NuxtTime :datetime="post?.date" :relative="true" />
        (<NuxtTime :datetime="post?.date" />)
      </p>
    </div>

    <div class="max-w-none border-b border-neutral-700 md:p-6">
      <ContentRenderer :value="post" class="prose-content space-y-3" />
    </div>
  </template>
</template>

<script setup lang="ts">
const route = useRoute()

const path = `/blog/${route.params.article}`

const { data: post } = await useAsyncData(`blog-${path}`, () =>
  queryCollection('blog').path(path).first()
)

if (!post.value) {
  throw createError({
    statusCode: 404,
    statusMessage: 'Page not found',
  })
}

useSeoMeta({
  title: post.value?.title,
  description: post.value?.description,
  ogType: 'article',
  ogTitle: `${post.value?.title} - Blueprint`,
  ogDescription: post.value?.description,
})

definePageMeta({
  layout: 'blog',
})

defineOgImageComponent('Descriptive', {
  scope: 'Blog',
})
</script>
