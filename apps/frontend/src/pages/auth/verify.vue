<template>
  <form
    @submit.prevent="handleVerify"
    class="w-full divide-y divide-neutral-700 border-y border-neutral-700"
  >
    <div class="p-4">
      <h1 class="!text-4xl">Verify your email</h1>
    </div>
    <div class="space-y-4 p-4">
      <ElementsInlinecard v-if="errors.incorrect">
        Something went wrong. Please double-check your verification code.
      </ElementsInlinecard>
      <ElementsInlinecard v-if="errors.resendError">
        Couldn't resend verification email, try again later.
      </ElementsInlinecard>
      <p class="text-default-font">
        We've sent a verification code to
        <span class="monospace-body"> {{ user?.email_pending }} </span>.
      </p>
      <ElementsFormInput
        v-model="form.token"
        name="token"
        type="text"
        :rules="[
          validationRules.required(),
          validationRules.minLength(16),
          validationRules.maxLength(16),
        ]"
        :required="true"
        leading-icon="memory:pound"
        autocomplete="nickname"
        placeholder="Verification code"
        :disabled="loading"
        @validate="
          (isValid: boolean) => handleFieldValidation('token', isValid)
        "
      />
      <p class="text-default-font/50">
        Used the wrong email? Update it in your
        <NuxtLink to="/app/account" class="text-link">account settings</NuxtLink
        >.
      </p>
    </div>
    <div
      class="flex flex-col divide-y divide-neutral-700 md:flex-row md:divide-x md:divide-y-0"
    >
      <button
        :disabled="fieldValidation.token == false || loading"
        type="submit"
        class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
      >
        <span class="text-xl font-semibold"> Submit </span>
        <Icon name="memory:chevron-right" mode="svg" :size="24" />
      </button>
      <NuxtLink>
        <button
          :disabled="loading"
          @click="handleResend"
          type="button"
          class="text-default-font hover:text-brand-50 w-full cursor-pointer text-nowrap bg-neutral-950 px-4 py-3 text-left text-xl font-semibold transition-colors hover:bg-neutral-900 md:w-auto"
        >
          Resend email
        </button>
      </NuxtLink>
    </div>
  </form>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'auth',
  middleware: 'user-unverified',
})

const { user, initializeAuth } = useAuth()
const { rules: validationRules } = useFormValidation()

const loading = ref(false)
const errors = ref({
  incorrect: false,
  resendError: false,
})
const fieldValidation = ref<Record<string, boolean>>({})
const form = ref({
  token: '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

const handleVerify = async () => {
  loading.value = true
  errors.value.incorrect = false
  errors.value.resendError = false

  try {
    await $fetch('/api/user/email/verify', {
      method: 'POST',
      body: form.value,
    })
  } catch {
    errors.value.incorrect = true
    loading.value = false
  } finally {
    await initializeAuth()
    await navigateTo('/app')
  }
}

const handleResend = async () => {
  loading.value = true
  errors.value.incorrect = false
  errors.value.resendError = false

  try {
    await $fetch('/api/user/email', {
      method: 'PATCH',
      body: {
        email: user.value?.email_pending,
        captcha: null,
      },
    })
  } catch {
    errors.value.resendError = true
    loading.value = false
  } finally {
    loading.value = false
  }
}
</script>
