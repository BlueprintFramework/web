<template>
  <div v-if="data">
    <div
      class="flex divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 xl:divide-x"
    >
      <div class="bg-stripes flex-1/6 hidden xl:block"></div>
      <div class="xl:flex-4/6 flex-[100%]">
        <NuxtImg
          :src="`/img/guides/thumbnails/${data.thumbnail || 'default.jpeg'}`"
          :width="1280"
          :height="275"
          class="w-full border-b border-neutral-700 object-cover"
        />
        <div class="space-y-3 border-b border-neutral-700 p-4 md:p-8">
          <h1 class="!text-4xl">{{ data.title || 'Untitled' }}</h1>
          <span class="opacity-75">
            {{ data.description }}
          </span>
        </div>
        <div class="prose prose-invert max-w-none p-4 md:p-8">
          <ContentRenderer :value="data" />
        </div>
      </div>
      <div class="bg-stripes flex-1/6 hidden xl:block"></div>
    </div>
  </div>
</template>

<script setup>
const route = useRoute()

const slug = route.params.slug
const slugArray = Array.isArray(slug) ? slug : [slug]
const path = `/guides/${slugArray.join('/')}`

const { data } = await useAsyncData(`guide-${path}`, () =>
  queryCollection('guides').path(path).first()
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
