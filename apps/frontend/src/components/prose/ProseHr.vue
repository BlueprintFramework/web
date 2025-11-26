<template>
  <div class="my-4 flex w-full items-center gap-2">
    <div class="flex-1/2">
      <div
        class="bg-linear-to-r h-[1px] w-full from-neutral-800 to-neutral-500"
      />
    </div>
    <div>
      <BrandEmblem class="text-neutral-500" />
    </div>
    <div class="flex-1/2">
      <div
        class="bg-linear-to-l h-[1px] w-full from-neutral-800 to-neutral-500"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
