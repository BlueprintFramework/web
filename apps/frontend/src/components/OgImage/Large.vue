<script setup lang="ts">
withDefaults(
  defineProps<{
    title?: string
    description?: string
  }>(),
  {
    title: 'title',
    description: '',
  }
)
</script>

<template>
  <div
    class="flex h-full w-full flex-col justify-center gap-8 bg-cover p-16"
    style="background-image: url(/img/ogbg.jpeg); background-size: cover"
  >
    <div class="flex flex-col items-center">
      <div class="flex items-center gap-6">
        <BrandLimitedemblem fill="white" :size="64" />
        <span class="text-6xl text-white">
          <span class="font-bold" style="font-family: 'Roboto'">Blueprint</span>
        </span>
      </div>
    </div>
    <div class="flex flex-col items-center">
      <p
        class="mb-0 text-center text-7xl text-white"
        style="line-height: 85px; font-family: 'Roboto'"
      >
        The modding platform for Pterodactyl
      </p>
    </div>
  </div>
</template>
