<template>
  <template v-if="data?.extension">
    <div
      class="flex flex-col gap-2 md:flex-row md:items-center md:justify-between"
    >
      <div class="flex items-center gap-2">
        <span class="h1"> {{ data.extension.name }} </span>
        <ElementsTextbadge
          v-if="user?.admin && user?.id != data.extension.author.id"
          :label="`${data.extension.author.name}`"
          icon="memory:account"
        />
      </div>
      <div class="flex items-center gap-2">
        <ElementsButton
          v-if="data.extension.status == 'pending'"
          class="max-md:w-full"
        >
          Submit for review
        </ElementsButton>
        <ElementsButton
          v-else-if="data.extension.status == 'ready'"
          class="max-md:w-full"
          :disabled="true"
        >
          <div class="flex items-center gap-1.5">
            <Icon name="pixelarticons:clock" />
            <span>Submit for review</span>
          </div>
        </ElementsButton>
        <ElementsButton class="max-md:w-full"> Save changes </ElementsButton>
      </div>
    </div>

    <ElementsInlinecard v-if="data.extension.deny_reason">
      Your extension submission was rejected for the following reason(s):
      <span class="text-default-font/60 italic">
        {{ data.extension.deny_reason }}
      </span>
      Please correct this/these issue(s) and re-submit the extension for review.
    </ElementsInlinecard>

    <div class="flex flex-col gap-5 xl:flex-row">
      <div
        class="xl:flex-2/3 overflow-hidden rounded-3xl border border-neutral-700"
      >
        <div
          class="h-50 flex w-full flex-col justify-between bg-cover bg-center bg-no-repeat xl:h-full"
          :style="`background-image: url(${data.extension.banner.fullres});`"
        >
          <div
            class="flex items-center gap-2 border-b border-neutral-700 bg-neutral-950/70 p-4 backdrop-blur-2xl"
          >
            <Icon name="memory:image" :size="28" />
            <span class="h2"> Banner </span>
          </div>
          <div class="flex w-full flex-col items-end justify-end p-4">
            <div>
              <ElementsButton>
                <div class="flex items-center gap-1.5">
                  <Icon name="pixelarticons:upload" />
                  <span>Upload banner</span>
                </div>
              </ElementsButton>
            </div>
          </div>
        </div>
      </div>
      <div class="xl:flex-1/3">
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
                validationRules.uniqueExtensionIdentifier(
                  data.extension.identifier
                ),
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
    </div>

    <div class="rounded-3xl border border-neutral-700">
      <div
        class="flex items-center justify-between gap-2 border-b border-neutral-700 p-4"
      >
        <div class="flex items-center gap-2">
          <Icon name="memory:text-image" :size="28" />
          <span class="h2"> Description </span>
        </div>
        <div>
          <SvgIconMarkdown :size="32" class="text-neutral-500" />
        </div>
      </div>
      <div class="p-4">
        <div
          class="grid grid-rows-2 overflow-hidden rounded-2xl border border-neutral-700 xl:grid-cols-2 xl:grid-rows-1"
        >
          <ElementsFormTextbox
            v-model="form.description"
            :rows="12"
            class="rounded-none border-0 font-mono"
            :placeholder="`(ﾉ*･_･)ﾉ \\\n**markdown** is supported`"
          />
          <div
            class="border-t border-neutral-700 p-4 xl:border-s xl:border-t-0"
          >
            <template v-if="!form.description || form.description == ''">
              <p>(ﾉ*･_･)ﾉ</p>
              <p><b>markdown</b> is supported</p>
            </template>
            <MDC
              v-else
              class="prose-content"
              :value="form.description"
              :parser-options="{
                rehype: { options: { allowDangerousHtml: false } },
              }"
            />
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
}>({
  identifier: '',
  name: '',
  platforms: {},
  summary: '',
  type: 'extension',
  unlisted: true,
  description: '',
})

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
