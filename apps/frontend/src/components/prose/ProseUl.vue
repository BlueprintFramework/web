<template>
  <ul class="my-2 list-disc ps-4">
    <slot />
  </ul>
</template>

<script setup lang="ts">
defineProps<{
  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
