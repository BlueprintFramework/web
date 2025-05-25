<template>
  <div class="h-[128px] w-[256px]">
    <NuxtImg :src="props.src" :height="128" :width="256" class="rounded-2xl" />
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  name: {
    type: String,
    required: true,
  },
  src: {
    type: String,
    required: true,
  },
  href: {
    type: String,
    required: true,
  },
})
</script>
