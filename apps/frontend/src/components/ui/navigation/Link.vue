<template>
  <NuxtLink :to="props.to" :target="props.target">
    <ElementsButtonSmall :label="props.label" :class="props.class" />
  </NuxtLink>
</template>

<script setup lang="ts">
const props = defineProps<{
  class?: string
  label: string
  to: string
  target?:
    | '_blank'
    | '_parent'
    | '_self'
    | '_top'
    | (string & {})
    | null
    | undefined
}>()
</script>
