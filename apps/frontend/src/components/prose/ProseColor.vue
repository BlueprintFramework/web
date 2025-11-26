<template>
  <p>hi</p>
</template>

<script setup lang="ts">
defineProps<{
  name?: string
  rgb: string

  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
