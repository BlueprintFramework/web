<template>
  <code
    :class="[
      $props.color == 'brand' ? 'text-brand-50 bg-brand-900' : '',
      $props.color == 'green' ? 'bg-emerald-950 text-emerald-400' : '',
      $props.color == 'red' ? 'bg-red-950 text-red-400' : '',
    ]"
  >
    <slot />
  </code>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    color: 'brand' | 'green' | 'red'

    // Disable class and style props
    class?: string
    style?: string
  }>(),
  {
    color: 'brand',
  }
)
</script>
