<template>
  <div class="space-y-8 md:space-y-2 md:py-4">
    <NuxtLink to="/guides/admin/install" class="group block">
      <div
        class="flex w-full justify-between overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 transition-colors group-hover:bg-neutral-900"
      >
        <div class="px-4 py-2">
          <span class="text-brand-50 font-bold"> Install Blueprint </span>
          <span> on your Pterodactyl panel, it's free! </span>
        </div>
        <div
          class="group-hover:text-brand-50 flex items-center gap-1 border-s border-neutral-700 p-2 transition-colors md:px-4 md:py-2"
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
  <UiMarketingSponsors />

  <ElementsGridbackground />
</template>
