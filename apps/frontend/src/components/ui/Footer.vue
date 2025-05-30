<template>
  <div
    :class="props.class"
    class="flex flex-row items-center justify-between text-neutral-500"
  >
    <BrandEmblem />
    <span class="text-lg"> The modding platform for Pterodactyl </span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  class: {
    type: String,
    required: false,
  },
})
</script>
