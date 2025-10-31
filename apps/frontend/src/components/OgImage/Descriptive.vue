<script setup lang="ts">
withDefaults(
  defineProps<{
    title?: string
    description?: string
    scope?: string
  }>(),
  {
    title: 'title',
    description: 'description',
    scope: '',
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
      <div class="flex items-center gap-6">
        <BrandLimitedemblem fill="white" :size="64" />
        <span class="text-6xl text-white">
          <span class="font-bold">Blueprint</span>
        </span>
        <span class="text-6xl text-white"> {{ scope }} </span>
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
