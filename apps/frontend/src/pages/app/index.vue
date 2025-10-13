<template>
  <div class="flex items-center gap-2">
    <span class="h1"> Hi </span>
    <div
      class="flex items-center gap-1 rounded-full border border-neutral-700 bg-neutral-900 px-2 py-1 text-xl"
    >
      <Icon name="memory:account" mode="svg" />
      <span>
        {{ user?.name }}
      </span>
    </div>
  </div>

  <UiAppVerifyemail v-if="user?.email_pending != null" />

  <div
    class="flex flex-col divide-y divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 md:grid md:grid-cols-4 md:divide-x md:divide-y-0"
  >
    <NuxtLink
      to="/app/account"
      class="hover:text-brand-50 space-y-2 bg-neutral-950 p-4 transition-colors hover:bg-neutral-900"
    >
      <div class="flex items-center gap-1.5">
        <Icon name="memory:account-box" :size="28" mode="svg" />
        <span class="text-xl font-bold">Account</span>
      </div>
      <span class="text-default-font/75">
        Display name, password, email address, pronouns, 2-factor authentication
      </span>
    </NuxtLink>
    <NuxtLink
      to="/app/extensions"
      class="hover:text-brand-50 space-y-2 bg-neutral-950 p-4 transition-colors hover:bg-neutral-900"
    >
      <div class="flex items-center gap-1.5">
        <Icon name="memory:cube" :size="28" mode="svg" />
        <span class="text-xl font-bold">Extensions</span>
      </div>
      <span class="text-default-font/75">
        Create product, configure products, unlist products
      </span>
    </NuxtLink>
    <NuxtLink
      to="/app/stats"
      class="hover:text-brand-50 space-y-2 bg-neutral-950 p-4 transition-colors hover:bg-neutral-900"
    >
      <div class="flex items-center gap-1.5">
        <Icon name="memory:chart-bar" :size="28" mode="svg" />
        <span class="text-xl font-bold">Statistics</span>
      </div>
      <span class="text-default-font/75">
        View statistics, customize graphs, embed widgets
      </span>
    </NuxtLink>
    <div v-if="isDesktop" class="bg-stripes" />
  </div>
</template>

<script setup lang="ts">
import { useMediaQuery } from '@vueuse/core'

definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

const { user } = useAuth()
const isDesktop = useMediaQuery('(min-width: 768px)')
</script>
