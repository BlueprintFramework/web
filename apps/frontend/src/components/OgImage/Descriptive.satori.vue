<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    title?: string
    description?: string
    description64?: string
    scope?: string
  }>(),
  {
    title: 'title',
    description: '',
    description64: '',
    scope: '',
  }
)

const decodedDescription = computed(() => {
  if (props?.description64 != '') {
    return atob(props.description64)
  }
  return props?.description
})
</script>

<template>
  <div
    class="flex h-full w-full flex-col justify-between bg-cover p-16"
    style="background-image: url(/img/ogbg.jpeg); background-size: cover"
  >
    <div class="flex flex-col">
      <div class="flex items-center">
        <BrandLimitedemblem fill="white" :size="64" />
        <span
          class="text-6xl text-white"
          style="font-family: 'Roboto'; padding-left: 20px"
        >
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
        style="font-family: 'Roboto'; padding-left: 5px"
        v-if="decodedDescription"
      >
        {{ decodedDescription }}
      </p>
    </div>
  </div>
</template>
