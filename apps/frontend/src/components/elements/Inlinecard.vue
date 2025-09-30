<template>
  <div
    class="space-y-2 rounded-2xl border border-neutral-700 bg-neutral-950 p-4"
    :class="props.class"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  class?: string
}>()
</script>
