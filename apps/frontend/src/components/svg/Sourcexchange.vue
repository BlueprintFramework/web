<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 48 48"
    :width="props?.size || 24"
    :class="props?.class"
  >
    <path
      d="m23.999 47.927v-13.69l11.247 5.113v3.4923z"
      fill="currentColor"
    ></path>
    <path
      d="m23.999 47.927v-13.69l-11.247 5.113v3.4923z"
      fill="currentColor"
    ></path>
    <path
      d="m12.752 39.363v-13.69l11.247 5.113v3.4923z"
      fill="currentColor"
    ></path>
    <path
      d="m23.999 30.772v-13.69l-11.247 5.113v3.4923z"
      fill="currentColor"
    ></path>
    <path
      d="m23.999 30.772v-13.69l11.247 5.113v3.4923z"
      fill="currentColor"
    ></path>
    <path
      d="m35.246 22.181v-13.69l-11.247 5.113v3.4923z"
      fill="currentColor"
    ></path>
    <path d="m24 13.69v-13.69l11.247 5.113v3.4923z" fill="currentColor"></path>
    <path d="m24 13.69v-13.69l-11.247 5.113v3.4923z" fill="currentColor"></path>
  </svg>
</template>

<script setup lang="ts">
const props = defineProps<{
  class?: string
  size?: string | number
}>()
</script>
