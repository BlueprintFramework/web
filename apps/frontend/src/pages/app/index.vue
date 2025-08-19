<template>
  <div class="flex items-center gap-2">
    <span class="h1"> Hi </span>
    <div
      class="flex items-center gap-1.5 rounded-full border border-neutral-700 bg-neutral-900 px-2 py-1 text-xl"
    >
      <Icon name="memory:account" mode="svg" />
      <span>
        {{ user?.name }}
      </span>
    </div>
  </div>

  <UiAppVerifyemail v-if="user?.email_pending != null" />

  <div
    class="grid grid-cols-4 divide-x divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700"
  >
    <div class="space-y-2 p-4">
      <div class="flex items-center gap-1.5">
        <Icon name="memory:account-box" :size="28" mode="svg" />
        <span class="text-xl font-bold">Account</span>
      </div>
      <span class="opacity-75">
        Display name, password, email address, connections, 2-factor
        authentication
      </span>
    </div>
    <div class="space-y-2 p-4">
      <div class="flex items-center gap-1.5">
        <Icon name="memory:cube" :size="28" mode="svg" />
        <span class="text-xl font-bold">Extensions</span>
      </div>
      <span class="opacity-75">
        Create product, configure products, unlist products
      </span>
    </div>
    <div class="space-y-2 p-4">
      <div class="flex items-center gap-1.5">
        <Icon name="memory:chart-bar" :size="28" mode="svg" />
        <span class="text-xl font-bold">Statistics</span>
      </div>
      <span class="opacity-75">
        View statistics, customize graphs, embed widgets
      </span>
    </div>
    <div class="bg-stripes" />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth',
})

const { user } = useAuth()
</script>
