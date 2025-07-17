<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 100 250"
    :width="props?.size || 24"
    :class="props?.class"
  >
    <line
      x1="50"
      :y1="props?.type == 'start' ? 125 : 0"
      x2="50"
      y2="250"
      stroke="currentColor"
      stroke-width="2"
    />
    <circle cx="50" cy="125" r="12" fill="currentColor" />
  </svg>
</template>

<script setup lang="ts">
const props = defineProps<{
  class?: string
  size?: string | number
  type?: 'start' | 'default'
}>()
</script>
