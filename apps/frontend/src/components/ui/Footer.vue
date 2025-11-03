<template>
  <div class="pb-[var(--extend-footer)]">
    <div
      :class="props.class"
      class="text-default-font flex items-center justify-between pb-6"
    >
      <div class="flex gap-3">
        <BrandEmblem class="text-brand-50" />
        <div class="flex items-center gap-1">
          <Icon name="memory:alpha-c" />
          <span>
            <span>2023-2025 Emma (</span>
            <NuxtLink
              to="https://prpl.wtf"
              class="hover:text-brand-50 default underline"
              >prpl.wtf</NuxtLink
            >
            <span>) and contributors</span>
          </span>
        </div>
      </div>
      <span class="hidden text-lg md:inline">
        The modding platform for Pterodactyl
      </span>
    </div>

    <div class="grid grid-cols-4 gap-4">
      <div class="flex flex-col gap-1 text-neutral-400 decoration-transparent">
        <p class="text-default-font mb-2 text-xl font-bold">Blueprint</p>
        <NuxtLink to="/browse" class="text-link"> Extensions </NuxtLink>
        <NuxtLink to="/guides" class="text-link"> Guides </NuxtLink>
        <NuxtLink to="/guides" class="text-link"> Documentation </NuxtLink>
        <NuxtLink to="/api" :external="true" class="text-link">
          API docs
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
        <p class="text-default-font mb-2 text-xl font-bold">Framework</p>
        <NuxtLink to="/releases" class="text-link"> Release history </NuxtLink>
        <NuxtLink
          to="https://github.com/blueprintframework/framework"
          class="text-link"
        >
          GitHub repository
        </NuxtLink>
      </div>

      <div class="flex flex-col gap-1 text-neutral-400 decoration-transparent">
        <p class="text-default-font mb-2 text-xl font-bold">Social</p>
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
