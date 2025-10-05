<template>
  <h1>Statistics</h1>
  <p>Coming soon :)</p>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

const { user } = useAuth()
</script>
