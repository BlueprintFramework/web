<template>
  <td
    class="text-default-font/80 border-r border-neutral-700 px-4 py-3 text-sm last:border-r-0"
    :align="props.align"
  >
    <slot />
  </td>
</template>

<script setup lang="ts">
const props = defineProps<{
  align?: 'left' | 'center' | 'right' | 'justify' | 'char'

  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
