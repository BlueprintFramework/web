<template>
  <NuxtLink
    :to="props.to"
    :target="props.target"
    class="hover:text-brand-50 rounded-md border border-neutral-700 bg-neutral-900 px-2.5 py-0.5 transition-colors hover:bg-neutral-800"
    :class="props.class"
  >
    {{ props.label }}
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
