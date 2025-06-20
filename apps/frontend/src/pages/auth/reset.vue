<template>
  <form class="w-full divide-y divide-neutral-700 border-y border-neutral-700">
    <div class="p-4">
      <h1 class="!text-4xl">Account recovery</h1>
    </div>
    <div class="space-y-4 p-4">
      <UiFormInput
        v-model="form.email"
        name="email"
        type="email"
        :rules="[]"
        leading-icon="memory:email"
        auto-complete="email"
        placeholder="Email address"
        @validate="void"
      />

      <span class="text-default-font/50">
        Enter the email associated with your Blueprint account. If an account
        exists, we'll send you a password reset link.
      </span>
    </div>
    <button
      type="submit"
      class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
    >
      <span class="text-xl font-semibold"> Send password reset link </span>
      <Icon name="memory:chevron-right" mode="svg" :size="24" />
    </button>
  </form>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'auth',
})

const form = ref({
  email: '',
})
</script>
