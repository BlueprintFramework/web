<template>
  <h1>Account settings</h1>

  <client-only>
    <UiAppUserProfile />
    <UiAppUserPassword />
    <UiAppUserTwofactor />
    <UiAppUserSessions />
  </client-only>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user',
  layout: 'dashboard',
})
</script>
