<template>
  <div
    class="flex flex-col divide-neutral-700 max-lg:divide-y lg:flex-row lg:divide-x"
  >
    <div
      class="grid grid-cols-1 gap-6 p-6 max-lg:w-full sm:grid-cols-2 lg:w-[500px] lg:grid-cols-1 lg:grid-rows-2"
    >
      <NuxtLink
        to="/guides/admin/install"
        class="group relative block h-full cursor-pointer overflow-hidden rounded-2xl border border-neutral-700 outline-0 transition-colors hover:border-neutral-500 focus:border-neutral-500"
        @mousedown.prevent
      >
        <NuxtImg
          src="/img/bluething.jpeg"
          :width="200"
          :height="50"
          class="h-full w-full object-cover transition-all group-hover:scale-105 group-hover:opacity-75 group-focus:scale-105 group-focus:opacity-75"
        />
        <div
          class="absolute left-1/2 top-1/2 w-full -translate-x-1/2 -translate-y-1/2 transform"
        >
          <div class="flex flex-col items-center">
            <div class="flex items-center gap-1">
              <span
                class="group-hover:text-brand-50 group-focus:text-brand-50 text-xl font-bold transition-colors"
                >Install Blueprint</span
              >
              <Icon
                name="memory:chevron-right"
                class="group-hover:text-brand-50 group-focus:text-brand-50 transition-colors"
                :size="24"
              />
            </div>
          </div>
        </div>
      </NuxtLink>

      <NuxtLink
        to="/releases"
        class="group relative block h-full w-full cursor-pointer overflow-hidden rounded-2xl border border-neutral-700 outline-0 transition-colors hover:border-neutral-500 focus:border-neutral-500"
        @mousedown.prevent
      >
        <NuxtImg
          src="/img/releases/latest.png"
          :width="200"
          :height="50"
          class="h-full w-full object-cover transition-all group-hover:scale-105 group-hover:opacity-75 group-focus:scale-105 group-focus:opacity-75"
        />
        <div
          class="absolute left-1/2 top-1/2 w-full -translate-x-1/2 -translate-y-1/2 transform"
        >
          <div class="flex flex-col items-center">
            <div class="flex items-center gap-1">
              <span
                class="group-hover:text-brand-50 group-focus:text-brand-50 text-xl font-bold transition-colors"
                >Release notes</span
              >
              <Icon
                name="memory:chevron-right"
                class="group-hover:text-brand-50 group-focus:text-brand-50 transition-colors"
                :size="24"
              />
            </div>
          </div>
        </div>
      </NuxtLink>
    </div>
    <div
      class="grid w-full grid-cols-1 gap-4 p-6 sm:grid-cols-2 md:grid-cols-4"
    >
      <div class="flex flex-col gap-1 text-neutral-400 decoration-transparent">
        <p class="text-default-font mb-2 text-xl font-bold">Blueprint</p>
        <NuxtLink to="/browse" class="text-link"> Extensions </NuxtLink>
        <NuxtLink to="/guides" class="text-link"> Guides </NuxtLink>
        <NuxtLink to="/docs" class="text-link"> Documentation </NuxtLink>
        <NuxtLink to="/releases" class="text-link"> Releases </NuxtLink>
        <NuxtLink to="/blog" class="text-link"> Blog </NuxtLink>
      </div>

      <div class="flex flex-col gap-1 text-neutral-400 decoration-transparent">
        <p class="text-default-font mb-2 text-xl font-bold">Donate</p>
        <NuxtLink
          to="https://hcb.hackclub.com/donations/start/blueprint"
          class="text-link"
        >
          Donate via HCB
        </NuxtLink>
        <NuxtLink to="https://github.com/sponsors/blueprintframework" class="text-link">
          Donate via GitHub Sponsors
        </NuxtLink>
        <NuxtLink to="https://hcb.hackclub.com/blueprint" class="text-link">
          Transparent finances
        </NuxtLink>
      </div>

      <div class="flex flex-col gap-1 text-neutral-400 decoration-transparent">
        <p class="text-default-font mb-2 text-xl font-bold">Agreements</p>
        <NuxtLink to="/legal/terms" class="text-link">
          Terms of Service
        </NuxtLink>
        <NuxtLink to="/legal/privacy" class="text-link">
          Privacy Policy
        </NuxtLink>
        <NuxtLink to="/legal/conduct" class="text-link">
          Code of Conduct
        </NuxtLink>
      </div>

      <div class="flex flex-col gap-1 text-neutral-400 decoration-transparent">
        <p class="text-default-font mb-2 text-xl font-bold">More</p>
        <NuxtLink
          to="https://discord.com/servers/blueprint-1063548024825057451"
          class="text-link"
        >
          Discord community
        </NuxtLink>
        <NuxtLink to="https://bsky.app/profile/blueprint.zip" class="text-link">
          Bluesky profile
        </NuxtLink>
        <NuxtLink to="https://github.com/blueprintframework" class="text-link">
          GitHub organization
        </NuxtLink>
        <NuxtLink to="/api" :external="true" class="text-link">
          API documentation
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  class: {
    type: String,
    required: false,
  },
})
</script>
