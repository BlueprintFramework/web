<template>
  <h1>Extensions</h1>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

const { user } = useAuth()
</script>
