<template>
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
</template>

<script setup lang="ts">
const props = defineProps({
  class: {
    type: String,
    required: false,
  },
})
</script>
