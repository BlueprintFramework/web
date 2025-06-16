<template>
  <div :class="props.class">TBD</div>
</template>

<script setup lang="ts">
const props = defineProps({
  class: {
    class: String,
    required: false,
  },
})
</script>
