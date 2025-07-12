<template>
  <h1>Discover powerful extensions and themes</h1>
  <div
    class="grid grid-cols-1 gap-px overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 md:grid-cols-2 lg:grid-cols-3"
  >
    <template v-if="pending">
      <div v-for="n in 7" class="flex flex-col bg-neutral-950">
        <div class="h-100"></div>
      </div>
    </template>

    <UiMarketingExtensioncard
      v-else
      v-for="extension in extensions || []"
      :key="extension.id"
      :extension="extension"
      class="flex flex-col bg-neutral-950"
    />

    <div class="w-[calc(200%+1px)] bg-neutral-950">
      <div class="bg-stripes h-full w-full" />
    </div>
  </div>
</template>

<script setup lang="ts">
const { data: extensions, pending } = await useAsyncData<Extension[]>(
  'extensions',
  () => $fetch<Extension[]>('/api/extensions'),
  {
    lazy: true,
    server: false,
  }
)
</script>
