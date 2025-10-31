<template>
  <template v-if="data">
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
      <ContentRenderer :value="data" class="prose-content space-y-3" />
    </div>
  </template>

  <div
    class="absolute inset-0 top-[var(--nav-offset)] -z-10 h-[50vh] w-full bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:30px_30px]"
    style="background-position-x: -5px"
  >
    <div
      class="bg-linear-to-r h-full w-full from-transparent via-neutral-950 to-transparent"
    />
  </div>
</template>

<script setup lang="ts">
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
  ogTitle: `${data.value?.title} - Blueprint`,
  ogDescription: data.value?.description,
})

definePageMeta({
  layout: 'legal',
})

defineOgImageComponent('Descriptive', {
  scope: 'Agreements',
})
</script>
