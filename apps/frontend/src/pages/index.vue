<template>
  <div class="space-y-8 md:space-y-2 md:py-4">
    <NuxtLink
      to="/guides/admin/install"
      class="group block outline-0"
      @mousedown.prevent
    >
      <div
        class="flex w-full justify-between overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 transition-colors group-hover:bg-neutral-900 group-focus:border-neutral-500 group-focus:bg-neutral-900"
      >
        <div class="px-4 py-2">
          <span class="text-brand-50 font-bold"> Install Blueprint </span>
          <span> on your Pterodactyl panel, it's free! </span>
        </div>
        <div
          class="group-hover:text-brand-50 group-focus:text-brand-50 flex items-center gap-1 border-s border-neutral-700 p-2 transition-colors group-focus:border-neutral-500 md:px-4 md:py-2"
        >
          <span class="hidden md:inline"> Get started </span>
          <Icon name="memory:chevron-right" :size="20" />
        </div>
      </div>
    </NuxtLink>

    <span class="md:display h1 block xl:text-justify">
      <span class="text-gray-500">The</span>
      industry-leading modding platform
      <span class="text-gray-500"> for the Pterodactyl® panel </span>
    </span>
  </div>

  <UiMarketingHighlights />
  <UiMarketingMarquee />
  <UiMarketingHero />
  <UiMarketingRepositories />
  <UiMarketingFaq />
  <UiMarketingTestimonials v-if="isDesktop" class="hidden md:block" />
  <UiMarketingSponsors />

  <ElementsGridbackground />
</template>

<script setup lang="ts">
import { useMediaQuery } from '@vueuse/core'

const isDesktop = useMediaQuery('(min-width: 768px)')
</script>
