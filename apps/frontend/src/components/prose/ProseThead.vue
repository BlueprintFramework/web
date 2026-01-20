<template>
  <thead class="border-b border-neutral-700 bg-neutral-900">
    <slot />
  </thead>
</template>

<script setup lang="ts">
defineProps<{
  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
