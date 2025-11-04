<template>
  <h1>Statistics</h1>

  <div
    class="bg-stripes grid grid-cols-1 overflow-hidden rounded-3xl border border-neutral-700 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
  >
    <div
      class="flex items-center justify-between gap-3 border-neutral-700 bg-neutral-950 p-4 md:border-e"
    >
      <div class="flex items-center gap-3">
        <Icon name="memory:cube" :size="28" mode="svg" />
        <p class="text-xl">Extensions</p>
      </div>
      <div
        class="min-w-12 rounded-[13px] border border-neutral-700 bg-neutral-900 px-3 py-0.5 text-center text-lg"
      >
        <span>{{ data?.extensions.total || 0 }}</span>
      </div>
    </div>
  </div>

  <p>Statistics are coming soon :)</p>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

const { data } = await useAsyncData<{
  extensions: { total?: number; unlisted?: number }
}>(`stats`, () => $fetch(`/api/user`), {
  server: false,
})
</script>
