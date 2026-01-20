<template>
  <div
    class="my-4 grid grid-cols-2 gap-[1px] overflow-hidden rounded-2xl border border-neutral-700 bg-neutral-700 sm:grid-cols-3 md:grid-cols-4"
  >
    <ElementsProseExtraColor
      v-for="item in $props.content"
      :name="item.name"
      :rgb="item.rgb"
    />
    <div class="min-h-5 w-[calc(400%+3px)] bg-neutral-950">
      <div class="bg-stripes h-full w-full" />
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  content: {
    name: string
    rgb: string
  }[]

  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
