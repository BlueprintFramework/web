<template>
  <svg
    viewBox="0 0 3 3"
    xmlns="http://www.w3.org/2000/svg"
    :class="props.class"
    :width="size"
    :height="size"
    :fill="fill || 'currentColor'"
    alt="Blueprint Logo"
  >
    <rect
      v-for="cube in cubes"
      :key="cube.index"
      :x="cube.x"
      :y="cube.y"
      width="1"
      height="1"
      class="emblem-cube transition-opacity"
      :style="{
        '--emblem-delay': `${cube.index * 70}ms`,
      }"
    ></rect>
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

const cubes = [
  { x: 1, y: 0, index: 0 },
  { x: 0, y: 1, index: 0 },
  { x: 2, y: 1, index: 1 },
  { x: 1, y: 2, index: 1 },
  { x: 2, y: 2, index: 2 },
]
</script>

<style>
@keyframes emblemSequence {
  0% {
    opacity: 1;
  }
  40%,
  60% {
    opacity: 0.2;
  }
  100% {
    opacity: 1;
  }
}

.emblem-animate .emblem-cube {
  animation: emblemSequence 0.6s ease-in-out forwards;
  animation-delay: var(--emblem-delay);
}
</style>
