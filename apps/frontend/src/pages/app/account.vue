<template>
  <h1>Account settings</h1>

  <client-only>
    <UiAppUserProfile />
    <UiAppUserPassword />
    <UiAppUserSessions />
    <div
      class="grid grid-cols-1 gap-[1px] overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 lg:grid-cols-2"
    >
      <UiAppUserTwofactor />
      <UiAppUserGithub />
    </div>
  </client-only>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user',
  layout: 'dashboard',
})
</script>
