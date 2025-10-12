<template>
  <div
    class="min-h-30 flex flex-col items-center justify-between gap-4 rounded-3xl border border-neutral-700 bg-[url(/img/banners.png)] bg-cover bg-right bg-no-repeat p-4 sm:flex-row"
  >
    <div class="w-full space-y-1 sm:w-auto">
      <p class="text-2xl font-bold">What are we building next?</p>
      <p class="text-default-font/60">
        Give your next extension a head start by publishing it to Blueprint.
      </p>
    </div>
    <ElementsButton
      label="New extension"
      class="w-full sm:w-auto"
      @click="modalOpen.onboarding = true"
    />
  </div>

  <ElementsModal
    :is-open="modalOpen.onboarding"
    :closable="true"
    title="New extension"
    @close="modalOpen.onboarding = false"
  >
    <template #default>
      <div
        class="grid grid-cols-1 grid-rows-2 gap-4 sm:grid-cols-2 sm:grid-rows-1"
      >
        <ElementsButton
          class="min-h-20 w-full !p-4"
          @click="
            () => {
              modalOpen.onboarding = false
              modalOpen.info = true
              isDistributed = false
            }
          "
        >
          <div class="flex w-full flex-col items-center gap-2">
            <Icon name="pixelarticons:plus" :size="28" />
            <p>I'm building a new extension</p>
          </div>
        </ElementsButton>
        <ElementsButton
          class="min-h-20 w-full !p-4"
          @click="
            () => {
              modalOpen.onboarding = false
              modalOpen.info = true
              isDistributed = true
            }
          "
        >
          <div class="flex w-full flex-col items-center gap-2">
            <Icon name="pixelarticons:link" :size="28" />
            <p>I've already published my extension to distributors</p>
          </div>
        </ElementsButton>
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Cancel"
        class="w-full md:w-auto"
        @click="modalOpen.onboarding = false"
      />
    </template>
  </ElementsModal>

  <ElementsModal
    :is-open="modalOpen.info"
    :closable="true"
    title="New extension"
    @close="
      () => {
        modalOpen.info = false
        modalOpen.onboarding = true
      }
    "
  >
    <template #default>
      <div class="space-y-4">
        <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
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
            :requiredIcon="false"
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
            :requiredIcon="false"
            placeholder="myextension"
            :disabled="loading"
            @validate="
              (isValid: boolean) =>
                handleFieldValidation('extension_identifier', isValid)
            "
          />
        </div>

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
          :requiredIcon="false"
          placeholder="This is my extension :)"
          :disabled="loading"
          @validate="
            (isValid: boolean) =>
              handleFieldValidation('extension_summary', isValid)
          "
        />

        <ElementsFormExtensiontype v-model="form.type" />
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Back"
        class="w-full md:w-auto"
        @click="
          () => {
            modalOpen.info = false
            modalOpen.onboarding = true
          }
        "
      />
      <ElementsButton
        :label="isDistributed ? 'Next' : 'Create'"
        :disabled="
          fieldValidation.extension_name == false ||
          fieldValidation.extension_identifier == false ||
          fieldValidation.extension_summary == false ||
          form.name == '' ||
          form.identifier == '' ||
          form.summary == '' ||
          (form.type != 'extension' && form.type != 'theme') ||
          loading
        "
        @click="handleCreate"
        type="submit"
        class="order-first w-full md:order-[unset] md:w-auto"
      />
    </template>
  </ElementsModal>

  <ElementsModal
    :is-open="modalOpen.platforms"
    :closable="true"
    title="New extension"
    @close="
      () => {
        modalOpen.platforms = false
        modalOpen.info = true
      }
    "
  >
    <template #default>
      lorem ipsum dolor whatever the fuck comes next
    </template>

    <template #footer>
      <ElementsButton
        label="Back"
        class="w-full md:w-auto"
        @click="
          () => {
            modalOpen.platforms = false
            modalOpen.info = true
          }
        "
      />
      <ElementsButton
        label="Create"
        class="order-first w-full md:order-[unset] md:w-auto"
        type="submit"
        @click="handleCreate"
      />
    </template>
  </ElementsModal>
</template>

<script setup lang="ts">
const { rules: validationRules } = useFormValidation()

const loading = ref(false)
const isDistributed = ref(false)
const fieldValidation = ref<Record<string, boolean>>({})
const modalOpen = ref({
  onboarding: false,
  info: false,
  platforms: false,
})
const form = ref<{
  identifier: string
  name: string
  platforms: ExtensionPlatforms
  summary: string
  type: ExtensionType
  unlisted: boolean
}>({
  identifier: '',
  name: '',
  platforms: {},
  summary: '',
  type: 'extension',
  unlisted: true,
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

const handleCreate = () => {
  form.value.unlisted = isDistributed.value == true

  if (isDistributed.value && !form.value.platforms[0]) {
    modalOpen.value.info = false
    modalOpen.value.platforms = true
    return
  }

  console.log('boop')
}
</script>
