<template>
  <ol class="my-2 list-decimal ps-4">
    <slot />
  </ol>
</template>

<script setup lang="ts">
defineProps<{
  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
