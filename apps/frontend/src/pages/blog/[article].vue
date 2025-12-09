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

    <div class="max-w-none border-b border-neutral-700 p-4 md:p-6">
      <ContentRenderer :value="post" class="prose-content space-y-3" />
    </div>

    <div class="flex justify-center border-b border-neutral-700">
      <div
        class="inline-flex flex-col divide-y divide-neutral-700 border-x border-neutral-700 md:flex-row md:divide-x md:divide-y-0"
      >
        <div
          class="inline-flex items-center justify-center gap-1 px-5 py-3 max-md:w-[90vw] md:gap-3 md:px-7"
        >
          <Icon name="memory:account" :size="36" class="max-md:size-5!" />
          <div class="hidden md:block">
            <p class="text-default-font/60">Post author</p>
            <p class="h2">{{ post.author }}</p>
          </div>
          <div class="md:hidden">
            <p class="text-default-font/60">
              Post written by
              <span class="text-default-font">
                {{ post.author }}
              </span>
            </p>
          </div>
        </div>
        <div
          class="inline-flex items-center justify-center gap-1.5 px-5 py-3 max-md:w-[90vw] md:gap-3 md:px-7"
        >
          <Icon name="memory:clock" :size="34" class="max-md:size-5!" />
          <div class="hidden md:block">
            <p class="text-default-font/60">Published on</p>
            <p class="h2">
              <NuxtTime :datetime="post?.date" />
            </p>
          </div>
          <div class="md:hidden">
            <p class="text-default-font/60">
              Published on
              <span class="text-default-font">
                <NuxtTime :datetime="post?.date" />
              </span>
            </p>
          </div>
        </div>
      </div>
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
