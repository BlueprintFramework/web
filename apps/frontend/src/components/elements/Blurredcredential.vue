<template>
  <div class="rounded-2xl border border-neutral-700 p-2">
    <div class="group me-2 rounded-2xl font-mono">
      <span
        class="wrap-break-word blur-sm transition-all group-hover:blur-none"
      >
        {{ props.content }}
      </span>
    </div>
    <span
      class="text-default-font/50 float-end me-2 bg-neutral-950 px-1 text-xs"
    >
      (hover to reveal)
    </span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  content: string
}>()
</script>
