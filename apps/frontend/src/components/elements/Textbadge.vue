<template>
  <div
    class="flex items-center gap-0.5 rounded-full border border-neutral-700 bg-neutral-900 px-2 py-1 text-xl"
  >
    <Icon v-if="props.icon" :name="props.icon" mode="svg" />
    <span v-if="props.label" class="px-0.5">
      {{ props.label }}
    </span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  icon?: string
  label?: string
}>()
</script>
