<template>
  <form
    @submit.prevent="handleRegister"
    class="w-full divide-y divide-neutral-700 border-y border-neutral-700"
  >
    <div class="p-4">
      <h1 class="!text-4xl">Hi there!</h1>
    </div>
    <div class="space-y-4 p-4">
      <UiFormInput
        v-model="form.displayName"
        name="displayname"
        type="text"
        :rules="[validationRules.required(), validationRules.name()]"
        :required="true"
        leading-icon="memory:user"
        placeholder="Display name"
        :disabled="loading"
        @validate="(event) => handleFieldValidation('displayName', event)"
      />
      <UiFormInput
        v-model="form.email"
        name="email"
        type="email"
        :rules="[validationRules.required(), validationRules.email()]"
        :required="true"
        leading-icon="memory:email"
        auto-complete="email"
        placeholder="Email address"
        :disabled="loading"
        @validate="(event) => handleFieldValidation('email', event)"
      />
      <UiFormInput
        v-model="form.password"
        name="password"
        type="password"
        :rules="[validationRules.required(), validationRules.password()]"
        :required="true"
        leading-icon="memory:key"
        auto-complete="password"
        placeholder="Password"
        :disabled="loading"
        @validate="(event) => handleFieldValidation('password', event)"
      />

      <span class="text-default-font/50">
        By creating an account, you acknowledge that you have read and agree to
        our
        <NuxtLink to="/legal/terms" class="text-link"
          >Terms of Service</NuxtLink
        >
        and
        <NuxtLink to="/legal/privacy" class="text-link"
          >Privacy Policy</NuxtLink
        >
      </span>
    </div>
    <button
      :disabled="
        !fieldValidation.displayName ||
        !fieldValidation.email ||
        !fieldValidation.password ||
        loading
      "
      type="submit"
      class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
    >
      <div class="text-xl font-semibold">
        <span>Sign up</span>
      </div>
      <Icon name="memory:chevron-right" mode="svg" :size="24" />
    </button>
  </form>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'auth',
  middleware: 'guest',
})

const { register } = useAuth()
const { rules: validationRules } = useFormValidation()

const loading = ref(false)
const fieldValidation = ref<Record<string, boolean>>({})
const form = ref({
  displayName: '',
  email: '',
  password: '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

const handleRegister = async () => {
  loading.value = true
  try {
    await register(
      form.value.email,
      form.value.password,
      form.value.displayName
    )
  } catch (error) {
    //TODO: Properly handle API errors
    console.error(error)
  } finally {
    loading.value = false
  }
}
</script>
