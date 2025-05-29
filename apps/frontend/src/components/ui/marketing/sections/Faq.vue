<template>
  <div
    :class="props.class"
    class="grid grid-cols-2 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
  >
    <div class="border-e border-neutral-700">
      <div
        class="flex flex-row justify-between border-b border-neutral-700 p-4"
      >
        <h2>What's Blueprint?</h2>
        <Icon name="memory:chevron-down" :size="24" />
      </div>
      <div
        class="flex flex-row justify-between border-b border-neutral-700 p-4"
      >
        <h2>Where can I discover and download extensions?</h2>
        <Icon name="memory:chevron-down" :size="24" />
      </div>
      <div
        class="flex flex-row justify-between border-b border-neutral-700 p-4"
      >
        <h2>Does Blueprint work with Pterodactyl forks?</h2>
        <Icon name="memory:chevron-down" :size="24" />
      </div>
      <div class="flex flex-row justify-between p-4">
        <h2>Are Blueprint's financials publicly available?</h2>
        <Icon name="memory:chevron-down" :size="24" />
      </div>
    </div>
    <div>test2</div>
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
