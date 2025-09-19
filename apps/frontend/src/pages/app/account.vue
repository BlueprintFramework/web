<template>
  <h1>Account settings</h1>

  <UiAppUserProfile />
  <UiAppUserPassword />
  <UiAppUserSessions />
  <UiAppUserTwofactor />
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user',
  layout: 'dashboard',
})

const { user } = useAuth()
</script>
