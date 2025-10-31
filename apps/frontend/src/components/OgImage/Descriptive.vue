<script setup lang="ts">
withDefaults(
  defineProps<{
    title?: string
    description?: string
    scope?: string
  }>(),
  {
    title: 'title',
    description: '',
    scope: '',
  }
)
</script>

<template>
  <div
    class="flex h-full w-full flex-col justify-between bg-cover p-16"
    style="background-image: url(/img/ogbg.jpeg); background-size: cover"
  >
    <div class="flex flex-col">
      <div class="flex items-center gap-6">
        <BrandLimitedemblem fill="white" :size="64" />
        <span class="text-6xl text-white">
          <span class="font-bold" style="font-family: 'Roboto'">Blueprint</span>
        </span>
        <span class="text-6xl text-white" style="font-family: 'Roboto'">
          {{ scope }}
        </span>
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
        style="font-family: 'Roboto'"
        v-if="description"
      >
        {{ description }}
      </p>
    </div>
  </div>
</template>
