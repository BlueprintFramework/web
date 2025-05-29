<template>
  <div
    :class="props.class"
    class="grid grid-cols-2 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
  >
    <div class="border-e border-neutral-700">
      <div
        class="hover:text-brand-50 flex flex-row justify-between border-b border-neutral-700 p-4 transition-colors hover:bg-neutral-900"
      >
        <h2>What's Blueprint?</h2>
        <Icon name="memory:chevron-down" :size="24" />
      </div>
      <div
        class="hover:text-brand-50 flex flex-row justify-between border-b border-neutral-700 p-4 transition-colors hover:bg-neutral-900"
      >
        <h2>Where can I discover and download extensions?</h2>
        <Icon name="memory:chevron-down" :size="24" />
      </div>
      <div
        class="hover:text-brand-50 flex flex-row justify-between border-b border-neutral-700 p-4 transition-colors hover:bg-neutral-900"
      >
        <h2>Does Blueprint work with Pterodactyl forks?</h2>
        <Icon name="memory:chevron-down" :size="24" />
      </div>
      <div
        class="hover:text-brand-50 flex flex-row justify-between p-4 transition-colors hover:bg-neutral-900"
      >
        <h2>What is Blueprint's financial structure?</h2>
        <Icon name="memory:chevron-down" :size="24" />
      </div>
    </div>
    <div class="relative overflow-hidden">
      <NuxtImg
        src="/img/gradient.jpeg"
        :width="1280"
        :height="300"
        class="h-full w-full object-cover"
      />
      <Icon
        name="memory:tooltip-above-help"
        mode="svg"
        :size="96"
        class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform"
      />
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
