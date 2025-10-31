<script setup lang="ts">
withDefaults(
  defineProps<{
    title?: string
    description?: string
  }>(),
  {
    title: 'title',
    description: 'description',
  }
)
</script>

<template>
  <div
    class="flex h-full w-full flex-col justify-between bg-cover p-16"
    style="
      background-image: url(https://s3.blueprint.zip/static/releases/releases2.jpeg);
      background-size: cover;
    "
  >
    <div class="flex flex-col">
      <div class="flex items-center gap-8">
        <BrandLimitedemblem fill="white" :size="72" />
        <span class="text-6xl font-bold text-white"> Blueprint </span>
      </div>
    </div>
    <div class="flex flex-col">
      <p
        class="text-8xl font-bold text-white"
        style="font-family: 'Funnel Display'"
      >
        {{ title }}
      </p>
      <p
        class="truncate text-nowrap text-4xl text-white"
        style="font-family: 'Funnel Display'"
      >
        {{ description }}
      </p>
    </div>
  </div>
</template>
