<template>
  <tbody class="divide-y divide-neutral-700">
    <slot />
  </tbody>
</template>

<script setup lang="ts">
defineProps<{
  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
