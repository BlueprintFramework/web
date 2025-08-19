<template>
  <div
    class="flex items-center rounded-3xl border border-red-800 bg-red-950 text-red-400"
  >
    <div class="p-4">
      <Icon name="memory:email" :size="24" mode="svg" />
    </div>
    <div class="border-s border-red-800 p-4">
      You need to verify your email address. We've sent an account verification
      email to
      <span class="monospace-body"> {{ user?.email_pending }} </span>.
    </div>
  </div>
</template>

<script setup lang="ts">
const { user } = useAuth()
</script>
