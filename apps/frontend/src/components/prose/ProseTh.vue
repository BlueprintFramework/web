<template>
  <th
    class="border-r border-neutral-700 px-4 py-3 text-left text-sm font-semibold last:border-r-0"
    :align="props.align"
    :class="props.class"
  >
    <slot />
  </th>
</template>

<script setup lang="ts">
const props = defineProps<{
  align?: 'left' | 'center' | 'right' | 'justify' | 'char'
  class?: string
}>()
</script>
