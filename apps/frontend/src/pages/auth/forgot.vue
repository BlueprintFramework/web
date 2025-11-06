<template>
  <form
    @submit.prevent="handleForgot"
    class="w-full divide-y divide-neutral-700 border-y border-neutral-700"
    v-if="!success"
  >
    <div class="p-4">
      <h1 class="!text-4xl">Forgot password</h1>
    </div>
    <div class="space-y-4 p-4">
      <ElementsInlinecard v-if="error">
        An unknown error occurred.
      </ElementsInlinecard>
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
      :disabled="fieldValidation.email == false || loading"
      type="submit"
      class="text-default-font focus:text-brand-50 hover:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
      @mousedown.prevent="handleForgot"
    >
      <span class="text-xl font-semibold"> Send password reset link </span>
      <Icon name="memory:chevron-right" mode="svg" :size="24" />
    </button>
  </form>

  <div
    v-if="success"
    class="w-full divide-y divide-neutral-700 border-y border-neutral-700"
  >
    <div class="p-4">
      <h1 class="!text-4xl">Check your email</h1>
    </div>
    <div class="space-y-4 p-4">
      <span class="text-default-font">
        We've sent a password reset link to
        <ProseCode>{{ form.email }}</ProseCode
        >. It may take a few minutes to arrive.
      </span>
    </div>
  </div>

  <ElementsTurnstilemodal
    v-model="turnstileModal.captchaValue.value"
    :is-open="turnstileModal.isOpen.value"
    ref="turnstileRef"
    @close="turnstileModal.close"
  />
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'auth',
})

const { rules: validationRules } = useFormValidation()

const turnstileModal = useTurnstileModal()
const turnstileRef = useTemplateRef('turnstileRef')

const loading = ref(false)
const error = ref(false)
const success = ref(false)
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
  success.value = false

  const result = await turnstileModal.show()
  if (!result.confirmed) {
    turnstileRef.value?.turnstile.reset()
    loading.value = false
    return
  }

  try {
    await $fetch('/api/auth/password/forgot', {
      method: 'POST',
      body: {
        email: form.value.email,
        captcha: turnstileModal.captchaValue.value,
      },
    })
    error.value = false
    success.value = true
  } catch {
    error.value = true
    success.value = false
  }

  turnstileRef.value?.turnstile.reset()
  loading.value = false
}
</script>
