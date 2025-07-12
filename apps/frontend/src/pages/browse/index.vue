<template>
  <h1>Discover powerful extensions and themes</h1>
  <div
    class="grid grid-cols-1 gap-px overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
  >
    <UiMarketingExtensioncard
      v-if="pending"
      v-for="n in 40"
      class="flex flex-col bg-neutral-950"
    />

    <UiMarketingExtensioncard
      v-else
      v-for="extension in extensions || []"
      :key="extension.id"
      :extension="extension"
      class="flex flex-col bg-neutral-950"
    />

    <div class="min-h-5 w-[calc(400%+3px)] bg-neutral-950">
      <div class="bg-stripes h-full w-full" />
    </div>
  </div>
</template>

<script setup lang="ts">
const { data: extensions, pending } = await useAsyncData<Extension[]>(
  'extensions',
  () => $fetch<Extension[]>('/api/extensions'),
  {
    server: false,
  }
)
</script>
