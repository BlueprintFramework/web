<template>
  <!-- prettier-ignore -->
  <ProsePre :code="$props.code" :language="$props.language" :filename="$props.filename"><slot/></ProsePre>
</template>

<script setup lang="ts">
defineProps<{
  code: string
  language?: string
  filename?: string

  // Disable class and style props
  class?: string
  style?: string
}>()
</script>
