<template>
  <NuxtLink :href="$props.href" :target="$props.target || ''" class="text-link">
    <slot />
  </NuxtLink>
</template>

<script setup lang="ts">
defineProps<{
  href: string
  target?: '_blank' | '_parent' | '_self' | '_top'

  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
