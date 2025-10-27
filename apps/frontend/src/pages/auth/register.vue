<template>
  <form
    @submit.prevent="handleRegister"
    class="w-full divide-y divide-neutral-700 border-y border-neutral-700"
  >
    <div class="p-4">
      <h1 class="!text-4xl">Hi there!</h1>
    </div>
    <div class="space-y-4 p-4">
      <ElementsInlinecard
        v-if="errors?.includes('user with name or email already exists')"
      >
        Email address or username is already in use.
      </ElementsInlinecard>
      <ElementsInlinecard v-if="errors?.includes('failed to create user')">
        Could not create user, try again later.
      </ElementsInlinecard>
      <ElementsFormInput
        v-model="form.displayName"
        name="displayname"
        type="text"
        :rules="[
          validationRules.required(),
          validationRules.name(),
          validationRules.minLength(3),
          validationRules.maxLength(15),
        ]"
        :required="true"
        leading-icon="memory:user"
        autocomplete="nickname"
        placeholder="Display name"
        :disabled="loading"
        @validate="
          (isValid: boolean) => handleFieldValidation('displayName', isValid)
        "
      />
      <ElementsFormInput
        v-model="form.email"
        name="email"
        type="email"
        :rules="[validationRules.required(), validationRules.email()]"
        :required="true"
        leading-icon="memory:email"
        autocomplete="email"
        placeholder="Email address"
        :disabled="loading"
        @validate="
          (isValid: boolean) => handleFieldValidation('email', isValid)
        "
      />
      <ElementsFormInput
        v-model="form.password"
        name="password"
        type="password"
        :rules="[validationRules.required(), validationRules.password()]"
        :required="true"
        leading-icon="memory:key"
        autocomplete="new-password"
        placeholder="Password"
        :disabled="loading"
        @validate="
          (isValid: boolean) => handleFieldValidation('password', isValid)
        "
      />

      <span class="text-default-font/50">
        By creating an account, you acknowledge that you have read and agree to
        our
        <!-- prettier-ignore -->
        <NuxtLink to="/legal/terms" class="text-link">Terms of Service</NuxtLink
        ><span>, </span>
        <!-- prettier-ignore -->
        <NuxtLink to="/legal/privacy" class="text-link">Privacy Policy</NuxtLink>
        and
        <!-- prettier-ignore -->
        <NuxtLink to="/legal/conduct" class="text-link">Code of Conduct</NuxtLink>
      </span>

      <NuxtTurnstile v-model="form.captcha" ref="turnstile" />
    </div>
    <button
      :disabled="
        fieldValidation.displayName == false ||
        fieldValidation.email == false ||
        fieldValidation.password == false ||
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

const turnstile = useTemplateRef('turnstile')
const loading = ref(false)
const errors = ref()
const fieldValidation = ref<Record<string, boolean>>({})
const form = ref({
  displayName: '',
  email: '',
  password: '',
  captcha: '',
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
      form.value.displayName,
      form.value.captcha
    )
  } catch (error) {
    console.error(error)
    errors.value = error
    turnstile.value?.reset()
  } finally {
    loading.value = false
  }
}
</script>
