<template>
  <form
    @submit.prevent="handleLogin"
    class="w-full divide-y divide-neutral-700 border-y border-neutral-700"
  >
    <div class="p-4">
      <h1 class="!text-4xl">Welcome back!</h1>
    </div>
    <div class="space-y-4 p-4">
      <Card v-if="errors?.includes('invalid name or password')">
        Invalid email or password. Double-check you've submitted the correct
        info or
        <NuxtLink to="/auth/reset" class="text-link"
          >recover your account here</NuxtLink
        >.
      </Card>
      <UiFormInput
        v-model="authForm.email"
        name="email"
        type="email"
        :rules="[validationRules.required(), validationRules.email()]"
        :required="true"
        leading-icon="memory:email"
        autocomplete="email"
        placeholder="Email address"
        :disabled="loading || checkpointData.authType == 'two_factor_required'"
        @validate="
          (isValid: boolean) => handleFieldValidation('email', isValid)
        "
      />
      <UiFormInput
        v-model="authForm.password"
        name="password"
        type="password"
        :rules="[validationRules.required()]"
        :required="true"
        leading-icon="memory:key"
        autocomplete="current-password"
        placeholder="Password"
        :disabled="loading || checkpointData.authType == 'two_factor_required'"
        @validate="
          (isValid: boolean) => handleFieldValidation('password', isValid)
        "
      />
      <UiFormInput
        v-model="checkpointForm.code"
        v-if="checkpointData.authType == 'two_factor_required'"
        name="code"
        type="text"
        :rules="[
          validationRules.required(),
          validationRules.minLength(6),
          validationRules.maxLength(10),
        ]"
        :required="true"
        leading-icon="memory:shield"
        autocomplete="one-time-code"
        placeholder="2FA code"
        :disabled="loading"
        @validate="(isValid: boolean) => handleFieldValidation('code', isValid)"
      />

      <span
        v-if="checkpointData.authType == 'two_factor_required'"
        class="text-default-font/50"
      >
        Use your 6-digit one time password or a 2FA recovery code.
      </span>
      <span v-else class="text-default-font/50">
        Forgot your password and/or lost access?
        <NuxtLink to="/auth/reset" class="text-link">
          Recover your account here
        </NuxtLink>
      </span>
    </div>
    <div
      class="flex flex-col divide-y divide-neutral-700 md:flex-row md:divide-x md:divide-y-0"
    >
      <button
        :disabled="
          (checkpointData.authType != 'two_factor_required' &&
            (fieldValidation.email == false ||
              fieldValidation.password == false)) ||
          (checkpointData.authType == 'two_factor_required' &&
            fieldValidation.code == false) ||
          loading
        "
        type="submit"
        class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
      >
        <span class="text-xl font-semibold"> Continue </span>
        <Icon name="memory:chevron-right" mode="svg" :size="24" />
      </button>
      <NuxtLink>
        <button
          :disabled="loading"
          type="button"
          class="text-default-font hover:text-brand-50 w-full cursor-pointer text-nowrap bg-neutral-950 px-4 py-3 text-left text-xl font-semibold transition-colors hover:bg-neutral-900 md:w-auto"
        >
          Authenticate with GitHub
        </button>
      </NuxtLink>
    </div>
  </form>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'auth',
  middleware: 'guest',
})

const { login, checkpoint, checkpointData } = useAuth()
const { rules: validationRules } = useFormValidation()

const loading = ref(false)
const errors = ref()
const fieldValidation = ref<Record<string, boolean>>({})
const authForm = ref({
  email: '',
  password: '',
})
const checkpointForm = ref({
  code: '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

const handleLogin = async () => {
  loading.value = true

  // [INFO] Two factor authentication
  if (checkpointData.value.authType == 'two_factor_required') {
    try {
      await checkpoint(checkpointForm.value.code)
    } catch (error) {
      console.error(error)
      errors.value = error
    } finally {
      loading.value = false
    }
    return
  }

  // [INFO] Sign in like normal
  try {
    await login(authForm.value.email, authForm.value.password)
  } catch (error) {
    console.error(error)
    errors.value = error
  } finally {
    loading.value = false
  }
}
</script>
