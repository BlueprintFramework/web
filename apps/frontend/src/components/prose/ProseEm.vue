<template>
  <em>
    <slot />
  </em>
</template>

<script setup lang="ts">
defineProps<{
  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
