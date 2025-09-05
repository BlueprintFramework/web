<template>
  <div v-if="isDev" class="text-red-400">Element is blocked from this view</div>
</template>

<script setup lang="ts">
const isDev = import.meta.dev
</script>
