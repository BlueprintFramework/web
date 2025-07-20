<template>
  <div
    :class="props.class"
    class="flex items-center justify-between pb-[var(--extend-footer)] text-neutral-500"
  >
    <div class="flex gap-3">
      <BrandEmblem />
      <div class="flex items-center gap-1">
        <Icon name="memory:alpha-c" />
        <span>
          <span>2023-2025 Emma (</span>
          <NuxtLink
            to="https://prpl.wtf"
            class="hover:text-brand-50 default underline"
            >prpl.wtf</NuxtLink
          >
          <span>)</span>
        </span>
      </div>
    </div>
    <span class="hidden text-lg md:inline">
      The modding platform for Pterodactyl
    </span>
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
