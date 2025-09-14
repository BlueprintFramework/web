<template>
  <h1>Account settings</h1>

  <UiAppUserProfile />
  <UiAppUserSecurity />
  <UiAppUserSessions />
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user',
  layout: 'dashboard',
})

const { user } = useAuth()
</script>
