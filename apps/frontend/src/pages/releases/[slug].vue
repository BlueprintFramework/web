<template>
  <template v-if="release">
    <div
      class="group grid w-full grid-cols-1 divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 outline-0 transition-colors focus-within:divide-neutral-500 focus-within:border-neutral-500 lg:grid-cols-2 lg:divide-x"
      @mousedown.prevent
    >
      <div class="flex flex-col justify-center p-6 transition-colors lg:p-12">
        <div class="w-full space-y-2 text-start">
          <p class="text-default-font/60 text-sm">
            Released
            <NuxtTime :datetime="release.released" :relative="true" />
          </p>
          <p>
            {{ release.summary || 'No summary provided' }}
          </p>
        </div>
      </div>
      <div
        class="min-h-20 overflow-hidden border-neutral-700 transition-colors group-focus:border-neutral-500 max-lg:order-first max-lg:border-b"
      >
        <div
          class="h-full w-full"
          :class="[
            release.latest ? 'bg-[url(/img/releases/latest.png)] bg-top' : '',
            release.supported && !release.latest
              ? 'bg-[url(/img/releases/release.png)] bg-bottom'
              : '',
            !release.supported && !release.latest
              ? 'bg-[url(/img/releases/unsupported.png)] bg-bottom'
              : '',
          ]"
        />
        <div
          class="relative -top-[100%] flex h-full w-full flex-col items-center justify-center p-6 lg:px-12"
        >
          <p
            class="display w-full truncate text-nowrap !text-start !text-3xl md:!text-4xl lg:!text-center lg:!text-5xl"
          >
            {{ release.version }}
          </p>
        </div>
      </div>
    </div>

    <div class="space-y-3">
      <h1 class="!text-4xl">
        {{ release.version || 'Untitled' }}
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
              Updated
              <NuxtTime :datetime="release?.released" :relative="true" />
            </span>
          </div>
          <span class="px-1.5 py-0.5">
            <NuxtTime :datetime="release?.released" />
          </span>
        </div>
      </div>
    </div>

    <div class="max-w-none">
      <ContentRenderer :value="release" class="prose-content space-y-3" />
    </div>
  </template>
</template>

<script setup lang="ts">
const route = useRoute()

const path = `/releases/${route.params.slug}`

const { data: release } = await useAsyncData(`release-${path}`, () =>
  queryCollection('releases').path(path).first()
)

if (!release.value) {
  throw createError({
    statusCode: 404,
    statusMessage: 'Page not found',
  })
}

useSeoMeta({
  title: release.value?.version,
  description: release.value?.summary,
})
</script>
