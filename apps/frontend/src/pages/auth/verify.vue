<template>
  <div class="w-full divide-y divide-neutral-700 border-y border-neutral-700">
    <div class="p-4">
      <h1 class="!text-4xl">Verify your email</h1>
    </div>
    <div class="space-y-4 p-4">
      <span class="text-default-font/50">
        You need to verify your email address to manage and publish extensions
        We've sent a verification code to
        <span class="monospace-body"> {{ user?.email_pending }} </span>. If
        that's not right, adjust your email in your
        <NuxtLink to="/app/account" class="text-link">account settings</NuxtLink
        >.
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'auth',
  middleware: 'user-unverified',
})

const { user } = useAuth()
</script>
