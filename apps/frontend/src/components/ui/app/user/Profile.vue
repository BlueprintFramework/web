<template>
  <div
    class="grid grid-cols-1 divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 md:grid-cols-2 md:divide-x"
  >
    <div class="divide-y divide-neutral-700">
      <div class="space-y-2 p-4">
        <h2>Profile</h2>
        <p>Update your Blueprint account information and email address.</p>
      </div>
      <div class="bg-stripes hidden h-full md:block" />
    </div>
    <form @submit.prevent="updateProfile" class="divide-y divide-neutral-700">
      <div class="grid grid-cols-1 gap-4 p-4 xl:grid-cols-2">
        <ElementsFormInput
          v-model="profileForm.name"
          label="Display name"
          description="Your display name, displayed side-wide and should be unique."
          name="name"
          type="text"
          :rules="[
            validationRules.required(),
            validationRules.name(),
            validationRules.minLength(3),
            validationRules.maxLength(15),
          ]"
          :required="true"
          leading-icon="memory:user"
          autocomplete="username"
          placeholder="Byte"
          :disabled="loading"
          @validate="
            (isValid: boolean) => handleFieldValidation('name', isValid)
          "
        />
        <ElementsFormInput
          v-model="profileForm.pronouns"
          label="Pronouns"
          description="Pronouns are displayed on your public profile, joke pronouns are not allowed."
          name="pronouns"
          type="text"
          :rules="[validationRules.pronouns(), validationRules.maxLength(22)]"
          leading-icon="memory:tag-text"
          placeholder="they/them"
          :disabled="loading"
          @validate="
            (isValid: boolean) => handleFieldValidation('pronouns', isValid)
          "
        />
        <ElementsFormInput
          v-model="accountForm.email"
          label="Email address"
          description="The email address associated with your Blueprint account."
          name="email"
          type="email"
          :rules="[validationRules.email(), validationRules.required()]"
          leading-icon="memory:email"
          placeholder="byte@blueprint.zip"
          :required="true"
          :disabled="loading"
          @validate="
            (isValid: boolean) => handleFieldValidation('email', isValid)
          "
        />
        <ElementsFormInput
          v-model="profileForm.support"
          label="Support URL"
          description="Link for users to get product support, in case you publish extensions."
          name="support"
          type="url"
          :rules="[validationRules.url()]"
          leading-icon="memory:tooltip-start-help"
          placeholder="mailto:example@example.com"
          :disabled="loading"
          @validate="
            (isValid: boolean) => handleFieldValidation('support', isValid)
          "
        />

        <NuxtTurnstile v-model="accountForm.captcha" ref="turnstile" />
      </div>
      <button
        :disabled="
          fieldValidation.name == false ||
          fieldValidation.pronouns == false ||
          fieldValidation.email == false ||
          fieldValidation.support == false ||
          loading
        "
        type="submit"
        class="disabled:text-default-font/40 hover:not-disabled:text-brand-50 hover:not-disabled:bg-neutral-900 flex w-full cursor-pointer items-center justify-between p-4 py-3 transition-colors disabled:cursor-not-allowed"
      >
        <span class="text-xl font-semibold"> Save changes </span>
        <Icon name="memory:floppy-disk" mode="svg" :size="24" />
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
const { user, initializeAuth } = useAuth()
const { rules: validationRules } = useFormValidation()

const turnstile = ref()
const loading = ref(false)
const errors = ref({
  profile: [],
  account: [],
})
const fieldValidation = ref<Record<string, boolean>>({})
const profileForm = ref({
  name: user.value?.name || '',
  pronouns: user.value?.pronouns || '',
  support: user.value?.support || '',
})
const accountForm = ref({
  email: user.value?.email || '',
  captcha: '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

const updateProfile = async () => {
  loading.value = true

  if (accountForm.value.email != user.value?.email) {
    try {
      await $fetch('/api/user/email', {
        method: 'PATCH',
        body: {
          email: accountForm.value.email,
          captcha: null,
        },
      })
    } catch (error) {
      console.error(error)
      //@ts-expect-error
      errors.value.account = error
      turnstile.value?.reset()
    }
  }

  try {
    await $fetch('/api/user', {
      method: 'PATCH',
      body: profileForm.value,
    })
  } catch (error) {
    console.error(error)
    //@ts-expect-error
    errors.value.profile = error
  } finally {
    await initializeAuth()
    loading.value = false
  }
}
</script>
