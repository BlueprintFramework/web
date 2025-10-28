<template>
  <NuxtLink :to="props.to" :target="props.target" class="group outline-0">
    <ElementsButtonSmall
      :label="props.label"
      :class="props.class"
      class="cursor-pointer group-focus:border-white"
      tabindex="-1"
      @mousedown.prevent
    />
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
