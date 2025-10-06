<template>
  <h1>Account settings</h1>

  <UiAppUserProfile />
  <UiAppUserPassword />
  <UiAppUserTwofactor />
  <UiAppUserSessions />
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user',
  layout: 'dashboard',
})
</script>
