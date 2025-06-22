<template>
  <form class="w-full divide-y divide-neutral-700 border-y border-neutral-700">
    <div class="p-4">
      <h1 class="!text-4xl">Hi there!</h1>
    </div>
    <div class="space-y-4 p-4">
      <div class="grid grid-cols-2 gap-4">
        <UiFormInput
          v-model="form.firstName"
          name="first_name"
          type="text"
          :rules="[validationRules.required()]"
          :required="true"
          leading-icon="memory:user"
          placeholder="First name"
          @validate="(event) => handleFieldValidation('firstName', event)"
        />
        <UiFormInput
          v-model="form.lastName"
          name="last_name"
          type="text"
          :rules="[validationRules.required()]"
          :required="true"
          placeholder="Last name"
          @validate="(event) => handleFieldValidation('lastName', event)"
        />
      </div>
      <UiFormInput
        v-model="form.email"
        name="email"
        type="email"
        :rules="[validationRules.required(), validationRules.email()]"
        :required="true"
        leading-icon="memory:email"
        auto-complete="email"
        placeholder="Email address"
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
const { rules: validationRules } = useFormValidation()

const fieldValidation = ref<Record<string, boolean>>({})
const form = ref({
  firstName: '',
  lastName: '',
  email: '',
  password: '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

definePageMeta({
  layout: 'auth',
})
</script>
