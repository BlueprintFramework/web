<template>
  <tr
    class="group transition-colors duration-200 hover:bg-neutral-800/20"
    :class="(props.to ? 'cursor-pointer ' : ' ') + props.class"
    :align="props.align"
    @click="handleClick"
  >
    <slot />
  </tr>
</template>

<script setup lang="ts">
const props = defineProps<{
  to?: string
  align?: 'left' | 'center' | 'right' | 'justify' | 'char'
  class?: string
}>()

const handleClick = () => {
  if (!props.to) return

  navigateTo(props.to)
}
</script>
