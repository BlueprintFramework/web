<template>
  <blockquote class="border-s-brand-50 border-s-5 pl-2 italic">
    <slot />
  </blockquote>
</template>

<script setup lang="ts">
defineProps<{
  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
