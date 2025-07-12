<template>
  <div v-if="props.extension" :class="props.class" class="">
    <span>{{ props.extension.identifier }}</span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  extension: Extension
  class?: string
}>()
</script>
