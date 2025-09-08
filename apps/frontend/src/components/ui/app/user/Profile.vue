<template>
  <div
    class="grid grid-cols-2 divide-x divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700"
  >
    <div class="divide-y divide-neutral-700">
      <div class="space-y-2 p-4">
        <h2>Profile</h2>
        <p>Your public profile on the Blueprint platform.</p>
      </div>
      <div class="bg-stripes h-full" />
    </div>
    <form @submit.prevent="updateProfile" class="divide-y divide-neutral-700">
      <div class="grid grid-cols-2 gap-4 p-4">
        <UiFormInput
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
        <UiFormInput
          v-model="profileForm.pronouns"
          label="Pronouns"
          description='Pronouns displayed on your profile. "Joke pronouns" are not allowed.'
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
        <UiFormInput
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
      </div>
      <button
        :disabled="
          fieldValidation.name == false ||
          fieldValidation.pronouns == false ||
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

const loading = ref(false)
const errors = ref()
const fieldValidation = ref<Record<string, boolean>>({})
const profileForm = ref({
  name: user.value?.name || '',
  pronouns: user.value?.pronouns || '',
  support: user.value?.support || '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

const updateProfile = async () => {
  loading.value = true

  try {
    await $fetch('/api/user', {
      method: 'PATCH',
      body: profileForm.value,
    })
  } catch (error) {
    console.error(error)
    errors.value = error
  } finally {
    await initializeAuth()
    loading.value = false
  }
}
</script>
