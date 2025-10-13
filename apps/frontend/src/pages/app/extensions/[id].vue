<template>
  <template v-if="data?.extension">
    <div class="flex items-center gap-2">
      <span class="h1"> {{ data.extension.name }} </span>
      <ElementsTextbadge
        v-if="user?.admin && user?.id != data.extension.author.id"
        :label="`${data.extension.author.name}`"
        icon="memory:account"
      />
    </div>

    <div class="flex flex-row gap-6">
      <div class="flex-1/3">
        <div class="rounded-3xl border border-neutral-700">
          <div class="flex items-center gap-2 border-b border-neutral-700 p-4">
            <Icon name="memory:text-box" :size="28" />
            <span class="h2"> General info </span>
          </div>
          <div class="space-y-4 p-4">
            <ElementsFormInput
              v-model="form.name"
              label="Name"
              description="Your extension's display name"
              name="extension_name"
              type="text"
              :rules="[
                validationRules.minLength(3),
                validationRules.maxLength(63),
                validationRules.required(),
              ]"
              :required="true"
              placeholder="My Extension"
              :disabled="loading"
              @validate="
                (isValid: boolean) =>
                  handleFieldValidation('extension_name', isValid)
              "
            />
            <ElementsFormInput
              v-model="form.identifier"
              label="Identifier"
              description="Your extension's identifier, must be unique"
              name="extension_identifier"
              type="text"
              :rules="[
                validationRules.extensionIdentifier(),
                validationRules.uniqueExtensionIdentifier(),
                validationRules.required(),
              ]"
              :required="true"
              placeholder="myextension"
              :disabled="loading"
              @validate="
                (isValid: boolean) =>
                  handleFieldValidation('extension_identifier', isValid)
              "
            />
            <ElementsFormInput
              v-model="form.summary"
              label="Summary"
              description="A short description of your extension"
              name="extension_summary"
              type="text"
              :rules="[
                validationRules.minLength(3),
                validationRules.maxLength(255),
                validationRules.required(),
              ]"
              :required="true"
              placeholder="This is my extension :)"
              :disabled="loading"
              @validate="
                (isValid: boolean) =>
                  handleFieldValidation('extension_summary', isValid)
              "
            />
            <ElementsExtensionType v-model="form.type" />
          </div>
        </div>
      </div>
      <div class="flex-2/3">
        <div class="rounded-3xl border border-neutral-700">
          <div class="flex items-center gap-2 border-b border-neutral-700 p-4">
            <Icon name="memory:text-image" :size="28" />
            <span class="h2"> Description </span>
          </div>
          <div class="space-y-4 p-4">
            <ElementsFormTextarea v-model="form.description" />
          </div>
        </div>
      </div>
    </div>
  </template>

  {{ data }}
</template>

<script setup lang="ts">
const route = useRoute()
const { user } = useAuth()
const { rules: validationRules } = useFormValidation()

const loading = ref(false)
const errors = ref(false)
const fieldValidation = ref<Record<string, boolean>>({})
const form = ref<{
  identifier: string
  name: string
  platforms: ExtensionPlatforms
  summary: string
  type: ExtensionType
  unlisted: boolean
  description: string
}>()

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

const { data } = await useAsyncData<{ extension: FullExtension }>(
  `user-extension-${route.params.id}`,
  () => $fetch(`/api/user/extensions/${route.params.id}`),
  {
    server: false,
  }
)

watch(
  () => data.value,
  (newData) => {
    if (newData?.extension) {
      form.value = {
        identifier: newData.extension.identifier || '',
        name: newData.extension.name || '',
        platforms: newData.extension.platforms || {},
        summary: newData.extension.summary || '',
        type: newData.extension.type || 'extension',
        unlisted: newData.extension.unlisted || true,
        description: newData.extension.description || '',
      }
    }
  },
  { immediate: true }
)
</script>
