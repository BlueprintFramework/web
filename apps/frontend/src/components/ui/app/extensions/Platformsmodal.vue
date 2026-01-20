<template>
  <ElementsModal
    :is-open="isOpen"
    :closable="true"
    title="Platforms"
    @close="handleClose"
  >
    <template #default>
      <div class="space-y-4">
        <!-- BuiltByBit -->
        <div class="rounded-lg border border-neutral-700 p-4">
          <div
            class="flex items-center justify-between"
            @click="localEnabled.BUILTBYBIT = !localEnabled.BUILTBYBIT"
          >
            <div class="flex items-center gap-2">
              <SvgBuiltbybit :size="20" />
              <span class="font-semibold">BuiltByBit</span>
            </div>
            <input
              v-model="localEnabled.BUILTBYBIT"
              type="checkbox"
              class="h-4 w-4"
            />
          </div>
          <ElementsFormInput
            v-if="localEnabled.BUILTBYBIT"
            v-model="localUrls.BUILTBYBIT"
            label="Product URL"
            name="builtbybit_url"
            type="url"
            :rules="[
              validationRules.required(),
              validationRules.url(),
              validationRules.platformUrl('BUILTBYBIT'),
            ]"
            :required="localEnabled.BUILTBYBIT"
            placeholder="https://builtbybit.com/resources/..."
            class="mt-3"
            @validate="
              (isValid: boolean) =>
                handleFieldValidation('builtbybit_url', isValid)
            "
          />
        </div>

        <!-- sourceXchange -->
        <div class="rounded-lg border border-neutral-700 p-4">
          <div
            class="flex items-center justify-between"
            @click="localEnabled.SOURCEXCHANGE = !localEnabled.SOURCEXCHANGE"
          >
            <div class="flex items-center gap-2">
              <SvgSourcexchange :size="20" />
              <span class="font-semibold">sourceXchange</span>
            </div>
            <input
              v-model="localEnabled.SOURCEXCHANGE"
              type="checkbox"
              class="h-4 w-4"
            />
          </div>
          <ElementsFormInput
            v-if="localEnabled.SOURCEXCHANGE"
            v-model="localUrls.SOURCEXCHANGE"
            label="Product URL"
            name="sourcexchange_url"
            type="url"
            :rules="[
              validationRules.required(),
              validationRules.url(),
              validationRules.platformUrl('SOURCEXCHANGE'),
            ]"
            :required="localEnabled.SOURCEXCHANGE"
            placeholder="https://sourcexchange.net/products/..."
            class="mt-3"
            @validate="
              (isValid: boolean) =>
                handleFieldValidation('sourcexchange_url', isValid)
            "
          />
        </div>

        <!-- GitHub -->
        <div class="rounded-lg border border-neutral-700 p-4">
          <div
            class="flex items-center justify-between"
            @click="localEnabled.GITHUB = !localEnabled.GITHUB"
          >
            <div class="flex items-center gap-2">
              <SvgGithub :size="20" />
              <span class="font-semibold">GitHub</span>
            </div>
            <input
              v-model="localEnabled.GITHUB"
              type="checkbox"
              class="h-4 w-4"
            />
          </div>
          <ElementsFormInput
            v-if="localEnabled.GITHUB"
            v-model="localUrls.GITHUB"
            label="Repository URL"
            name="github_url"
            type="url"
            :rules="[
              validationRules.required(),
              validationRules.url(),
              validationRules.platformUrl('GITHUB'),
            ]"
            :required="localEnabled.GITHUB"
            placeholder="https://github.com/user/repo"
            class="mt-3"
            @validate="
              (isValid: boolean) => handleFieldValidation('github_url', isValid)
            "
          />
        </div>
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Close"
        class="w-full md:w-auto"
        @click="handleClose"
      />
    </template>
  </ElementsModal>
</template>

<script setup lang="ts">
const { rules: validationRules } = useFormValidation()

interface Props {
  isOpen: boolean
  platforms: ExtensionPlatformUrls
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  save: [platforms: ExtensionPlatformUrls]
}>()

const fieldValidation = ref<Record<string, boolean>>({})

const localEnabled = ref<Record<string, boolean>>({
  BUILTBYBIT: false,
  SOURCEXCHANGE: false,
  GITHUB: false,
})

const localUrls = ref<Record<string, string>>({
  BUILTBYBIT: '',
  SOURCEXCHANGE: '',
  GITHUB: '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

watch(
  () => props.isOpen,
  (isOpen) => {
    if (isOpen) {
      localEnabled.value = {
        BUILTBYBIT: !!props.platforms.BUILTBYBIT,
        SOURCEXCHANGE: !!props.platforms.SOURCEXCHANGE,
        GITHUB: !!props.platforms.GITHUB,
      }

      localUrls.value = {
        BUILTBYBIT: props.platforms.BUILTBYBIT || '',
        SOURCEXCHANGE: props.platforms.SOURCEXCHANGE || '',
        GITHUB: props.platforms.GITHUB || '',
      }
    }
  },
  { immediate: true }
)

const handleClose = () => {
  const platforms: ExtensionPlatformUrls = {}

  if (
    localEnabled.value.BUILTBYBIT &&
    localUrls.value.BUILTBYBIT &&
    fieldValidation.value.builtbybit_url !== false
  ) {
    platforms.BUILTBYBIT = localUrls.value.BUILTBYBIT.endsWith('/')
      ? localUrls.value.BUILTBYBIT.slice(0, -1)
      : localUrls.value.BUILTBYBIT
  }

  if (
    localEnabled.value.SOURCEXCHANGE &&
    localUrls.value.SOURCEXCHANGE &&
    fieldValidation.value.sourcexchange_url !== false
  ) {
    platforms.SOURCEXCHANGE = localUrls.value.SOURCEXCHANGE.endsWith('/')
      ? localUrls.value.SOURCEXCHANGE.slice(0, -1)
      : localUrls.value.SOURCEXCHANGE
  }

  if (
    localEnabled.value.GITHUB &&
    localUrls.value.GITHUB &&
    fieldValidation.value.github_url !== false
  ) {
    platforms.GITHUB = localUrls.value.GITHUB.endsWith('/')
      ? localUrls.value.GITHUB.slice(0, -1)
      : localUrls.value.GITHUB
  }

  emit('save', platforms)
  emit('close')
}
</script>
