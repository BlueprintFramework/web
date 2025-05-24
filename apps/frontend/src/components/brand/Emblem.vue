<script setup lang="ts">
const props = defineProps({
  size: {
    type: Number,
    required: false,
  },
  class: {
    type: String,
    required: false,
  },
  fill: {
    type: String,
    required: false,
  },
})

const size = (props.size || 24) + 'px'
const fill = props.fill || 'currentColor'
</script>

<template>
  <svg
    viewBox="0 0 3 3"
    xmlns="http://www.w3.org/2000/svg"
    :class="props.class"
    :width="size"
    :height="size"
    alt="Blueprint Logo"
  >
    <rect x="1" y="0" width="1" height="1" :fill="fill" />
    <rect x="0" y="1" width="1" height="1" :fill="fill" />
    <rect x="2" y="1" width="1" height="1" :fill="fill" />
    <rect x="1" y="2" width="2" height="1" :fill="fill" />
  </svg>
</template>
