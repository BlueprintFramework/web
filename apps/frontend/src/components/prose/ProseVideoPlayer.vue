<template>
  <div class="my-4 first-line:mt-0">
    <video
      loop
      controls
      :width="props.width || undefined"
      :height="props.height || undefined"
      class="h-full w-full overflow-hidden rounded-2xl border border-neutral-700 object-cover"
    >
      <source :src="props.src" :type="props.type || 'video/mp4'" />
    </video>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    src: string
    type?: string
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
