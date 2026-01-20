<template>
  <div
    class="my-4 overflow-hidden rounded-2xl border border-neutral-700 first-line:mt-0"
  >
    <component
      :is="ImageComponent"
      :src="src"
      :alt="props.alt"
      :width="props.width"
      :height="props.height"
      class="w-full"
    />
  </div>
</template>

<script setup lang="ts">
import ImageComponent from '#build/mdc-image-component.mjs'

const props = withDefaults(
  defineProps<{
    src?: string
    alt?: string
    width?: string | number
    height?: string | number

    // Disable class and style props
    class?: string
    style?: string
  }>(),
  {
    src: '',
    alt: '',
  }
)
</script>
