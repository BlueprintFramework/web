<template>
  <form class="w-full divide-y divide-neutral-700 border-y border-neutral-700">
    <div class="p-4">
      <h1 class="!text-4xl">Welcome back!</h1>
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
      <UiFormInput
        v-model="form.password"
        name="password"
        type="password"
        :rules="[]"
        leading-icon="memory:key"
        auto-complete="password"
        placeholder="Password"
        @validate="void"
      />

      <span class="text-default-font/50">
        Forgot your password and/or lost access?
        <NuxtLink to="/auth/reset" class="text-link">
          Recover your account here
        </NuxtLink>
      </span>
    </div>
    <div class="flex divide-x divide-neutral-700">
      <button
        type="submit"
        class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
      >
        <span class="text-xl font-semibold"> Continue </span>
        <Icon name="memory:chevron-right" mode="svg" :size="24" />
      </button>
      <button
        class="text-default-font hover:text-brand-50 w-auto cursor-pointer text-nowrap bg-neutral-950 px-4 py-3 text-xl font-semibold transition-colors hover:bg-neutral-900"
      >
        Authenticate with GitHub
      </button>
    </div>
  </form>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'auth',
})

const form = ref({
  email: '',
  password: '',
})
</script>
