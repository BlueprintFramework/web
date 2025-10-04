<template>
  <form
    @submit.prevent="handleForgot"
    class="w-full divide-y divide-neutral-700 border-y border-neutral-700"
  >
    <div class="p-4">
      <h1 class="!text-4xl">Forgot password</h1>
    </div>
    <div class="space-y-4 p-4">
      <ElementsFormInput
        v-model="form.email"
        name="email"
        type="email"
        :rules="[validationRules.required(), validationRules.email()]"
        :required="true"
        leading-icon="memory:email"
        autocomplete="email"
        placeholder="Email address"
        @validate="
          (isValid: boolean) => handleFieldValidation('email', isValid)
        "
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
  middleware: 'guest',
})

const { rules: validationRules } = useFormValidation()

const loading = ref(false)
const error = ref()
const fieldValidation = ref<Record<string, boolean>>({})
const form = ref({
  email: '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

const handleForgot = async () => {
  loading.value = true
  error.value = false

  try {
    await $fetch('/api/auth/password/forgot', {
      method: 'POST',
      body: {
        email: form.value.email,
        captcha: null,
      },
    })
  } catch {
    error.value = true
    loading.value = false
  } finally {
    loading.value = false
  }
}
</script>
