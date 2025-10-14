<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 15 9"
    shape-rendering="crispEdges"
    :width="(props?.size || 24) + 'px'"
    :class="props?.class"
  >
    <rect x="1" y="0" width="13" height="2" fill="currentColor" />
    <rect x="0" y="1" width="1" height="7" fill="currentColor" />
    <rect x="14" y="1" width="1" height="7" fill="currentColor" />
    <rect x="1" y="2" width="1" height="7" fill="currentColor" />
    <rect x="2" y="2" width="1" height="5" fill="#000000" fill-opacity="0" />
    <rect x="3" y="2" width="3" height="1" fill="currentColor" />
    <rect x="6" y="2" width="1" height="5" fill="#000000" fill-opacity="0" />
    <rect x="7" y="2" width="3" height="2" fill="currentColor" />
    <rect x="10" y="2" width="1" height="5" fill="#000000" fill-opacity="0" />
    <rect x="11" y="2" width="3" height="2" fill="currentColor" />
    <rect x="3" y="3" width="1" height="2" fill="#000000" fill-opacity="0" />
    <rect x="4" y="3" width="1" height="1" fill="currentColor" />
    <rect x="5" y="3" width="1" height="2" fill="#000000" fill-opacity="0" />
    <rect x="4" y="4" width="1" height="2" fill="#000000" fill-opacity="0" />
    <rect x="7" y="4" width="1" height="5" fill="currentColor" />
    <rect x="8" y="4" width="2" height="1" fill="#000000" fill-opacity="0" />
    <rect x="11" y="4" width="2" height="1" fill="#000000" fill-opacity="0" />
    <rect x="13" y="4" width="1" height="5" fill="currentColor" />
    <rect x="3" y="5" width="1" height="4" fill="currentColor" />
    <rect x="5" y="5" width="1" height="4" fill="currentColor" />
    <rect x="8" y="5" width="1" height="4" fill="currentColor" />
    <rect x="9" y="5" width="1" height="1" fill="#000000" fill-opacity="0" />
    <rect x="11" y="5" width="1" height="1" fill="#000000" fill-opacity="0" />
    <rect x="12" y="5" width="1" height="4" fill="currentColor" />
    <rect x="4" y="6" width="1" height="3" fill="currentColor" />
    <rect x="9" y="6" width="1" height="3" fill="currentColor" />
    <rect x="11" y="6" width="1" height="3" fill="currentColor" />
    <rect x="2" y="7" width="1" height="2" fill="currentColor" />
    <rect x="6" y="7" width="1" height="2" fill="currentColor" />
    <rect x="10" y="7" width="1" height="2" fill="currentColor" />
  </svg>
</template>

<script setup lang="ts">
const props = defineProps<{
  class?: string
  size?: string | number
}>()
</script>
