<template>
  <div class="flex divide-x divide-neutral-700">
    <div class="w-[500px] p-6">
      <div
        class="relative h-20 overflow-hidden rounded-2xl border border-neutral-700"
      >
        <NuxtImg
          src="/img/releases/latest.png"
          :width="200"
          :height="80"
          class="h-full w-full object-cover"
        />
        <div
          class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform"
        >
          <div class="flex items-center gap-1">
            <span class="text-xl font-bold">Release notes</span>
            <Icon name="memory:chevron-right" />
          </div>
        </div>
      </div>
    </div>
    <div class="grid w-full grid-cols-3 gap-4 p-6">
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
