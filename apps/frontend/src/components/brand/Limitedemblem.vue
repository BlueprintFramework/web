<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :class="props.class"
    :width="size"
    :height="size"
    :fill="fill || 'currentColor'"
    viewBox="0 0 3 3"
  >
    <path d="M1 0h1v1H1zM0 1h1v1H0z" />
    <path d="M2 1h1v1H2zM1 2h1v1H1z" />
    <path d="M2 2h1v1H2z" />
  </svg>
</template>

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
</script>
