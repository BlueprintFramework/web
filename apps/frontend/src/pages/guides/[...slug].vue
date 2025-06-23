<template>
  <div v-if="data">
    <div
      class="lg:min-h-50 grid grid-cols-1 divide-neutral-700 overflow-hidden rounded-t-3xl border border-neutral-700 bg-neutral-950 lg:grid-cols-2 lg:divide-x"
    >
      <div class="flex h-full">
        <div class="space-y-3 self-center p-4 md:p-8">
          <h1 class="!text-4xl">{{ data.title || 'Untitled' }}</h1>
          <span class="opacity-75">
            {{ data.description }}
          </span>
        </div>
      </div>
      <div class="hidden lg:block">
        <NuxtImg
          :src="`/img/guides/thumbnails/${data.thumbnail || 'default.jpeg'}`"
          :width="1280"
          :height="350"
          class="h-full w-full object-cover"
        />
      </div>
    </div>

    <div
      class="grid grid-cols-1 divide-neutral-700 overflow-hidden rounded-b-3xl border border-t-0 border-neutral-700 bg-neutral-950 xl:grid-cols-2 xl:divide-x"
    >
      <div class="p-4 md:p-8">
        <div class="prose prose-invert max-w-none">
          <ContentRenderer :value="data" />
        </div>
      </div>
      <div class="bg-stripes hidden xl:block"></div>
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
