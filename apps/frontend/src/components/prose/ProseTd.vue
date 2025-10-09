<template>
  <td
    class="text-default-font/80 border-r border-neutral-700 px-4 py-3 text-sm last:border-r-0"
    :align="props.align"
    :class="props.class"
  >
    <slot />
  </td>
</template>

<script setup lang="ts">
const props = defineProps<{
  align?: 'left' | 'center' | 'right' | 'justify' | 'char'
  class?: string
}>()
</script>
