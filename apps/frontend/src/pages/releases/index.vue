<template>
  <NuxtLink
    v-if="latestRelease"
    class="group grid w-full cursor-pointer grid-cols-1 divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 outline-0 transition-colors hover:bg-neutral-900 focus:divide-neutral-500 focus:border-neutral-500 focus:bg-neutral-900 lg:grid-cols-2 lg:divide-x"
    :to="latestRelease.path"
    @mousedown.prevent
  >
    <div class="flex flex-col justify-center p-6 transition-colors lg:p-12">
      <div class="w-full space-y-2 text-start">
        <p class="text-default-font/60 text-sm">
          Released
          <NuxtTime :datetime="latestRelease.released" :relative="true" />
        </p>
        <p>
          {{ latestRelease.summary }}
        </p>
        <p
          class="text-link group-focus:text-brand-50 group-hover:text-brand-50 mt-6 flex items-center gap-0.5"
        >
          <span> View release </span>
          <Icon name="memory:chevron-right" />
        </p>
      </div>
    </div>
    <div
      class="min-h-20 overflow-hidden border-neutral-700 transition-colors group-focus:border-neutral-500 max-lg:order-first max-lg:border-b"
    >
      <div
        class="h-full w-full bg-[url(/img/releases/latest.png)] transition-all duration-300 group-hover:scale-105"
      />
      <div
        class="relative -top-[100%] flex h-full w-full flex-col items-center justify-center p-6 lg:px-12"
      >
        <p
          class="display w-full truncate text-nowrap !text-start !text-3xl md:!text-4xl lg:!text-center lg:!text-5xl"
        >
          {{ latestRelease.version }}
        </p>
      </div>
    </div>
  </NuxtLink>

  <div
    v-if="releases?.length"
    class="grid grid-cols-1 gap-[1px] overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 transition-colors focus-within:border-neutral-500 focus-within:bg-neutral-500 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
  >
    <NuxtLink
      v-for="release in releases"
      :to="release.path"
      class="group block bg-neutral-950 outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
      @mousedown.prevent
    >
      <div
        class="space-y-3 p-4 md:p-6"
        :class="
          release.supported
            ? 'bg-gradient-to-r from-neutral-900/80 to-transparent'
            : ''
        "
      >
        <div class="flex items-center gap-2">
          <span
            v-if="release.latest"
            class="rounded-full border border-neutral-700 bg-neutral-900 px-2 py-0.5 text-sm"
          >
            Latest
          </span>
          <span
            v-else-if="release.supported"
            class="rounded-full border border-neutral-700 bg-neutral-900 px-2 py-0.5 text-sm"
          >
            Supported
          </span>
          <span class="text-default-font/60 truncate text-nowrap text-sm">
            Released <NuxtTime :datetime="release.released" :relative="true" />
          </span>
        </div>

        <div>
          <p class="truncate text-nowrap text-lg font-bold">
            {{ release.version }}
          </p>

          <p class="truncate text-nowrap">
            {{ release.summary || 'This release does not have a summary' }}
          </p>
        </div>

        <p
          class="text-link group-focus:text-brand-50 group-hover:text-brand-50 flex items-center gap-0.5"
        >
          <span> View release </span>
          <Icon name="memory:chevron-right" />
        </p>
      </div>
    </NuxtLink>
    <div class="min-h-5 w-[calc(400%+3px)] bg-neutral-950">
      <div class="bg-stripes h-full w-full" />
    </div>
  </div>

  <div
    class="group flex flex-col divide-y divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 transition-colors focus-within:divide-neutral-500 focus-within:border-neutral-500"
  >
    <div class="grid grid-cols-1 transition-colors lg:grid-cols-2">
      <p class="p-4 lg:p-6">
        Blueprint has an extensive history of releases. Most older alpha and
        indev releases, however, no longer have changelogs attached to them.
      </p>
      <div
        class="bg-stripes hidden h-full w-full border-s border-neutral-700 transition-colors group-focus-within:border-neutral-500 lg:block"
      />
    </div>
    <div
      class="grid grid-cols-2 gap-[1px] bg-neutral-700 transition-colors group-focus-within:bg-neutral-500 md:grid-cols-4"
    >
      <NuxtLink
        v-for="release in legacyReleases"
        :to="`https://github.com/blueprintframework/framework/releases/${release}`"
        class="hover:text-brand-50 focus:text-brand-50 bg-neutral-950 p-2 outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
        @mousedown.prevent
      >
        {{ release }}
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
const { data: releases } = await useAsyncData('changelog-index', () => {
  return queryCollection('changelog').order('num', 'DESC').all()
})

const latestRelease = computed(() => {
  return releases.value?.find((log) => log.latest)
})

const legacyReleases = ref([
  'alpha-VKL',
  'alpha-83W',
  'alpha-L53',
  'alpha-YE5',
  'alpha-LV5',
  'alpha-4TK',
  'alpha-A9X',
  'alpha-Y4Y',
  'alpha-7FZ',
  'alpha-EV0',
  'alpha-5KB',
  'alpha-ML7',
  'alpha-T0R',
  'alpha-DB4',
  'alpha-XFC',
  'alpha-X34',
  'indev-AXN',
  'indev-8X2',
  'indev-1GG',
  'indev-OXG',
  'indev-M29',
  'indev-QNH',
  'indev-WI9',
  'indev-TC2',
  'indev-4N3',
  'indev-MS7',
  'indev-DW2',
  'indev-00A',
  'indev-D6G',
  'indev-7WA',
  'indev-YNM',
  'indev-CIX',
  'indev-T5W',
  'indev-R4U',
  'indev-MSA',
  'indev-IV8',
])
</script>
