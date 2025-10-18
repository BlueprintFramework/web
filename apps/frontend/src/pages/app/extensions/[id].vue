<template>
  <client-only>
    <template v-if="data?.extension">
      <div
        v-if="user?.admin && data.extension.status == 'ready'"
        class="flex justify-between rounded-3xl border border-neutral-700 max-md:flex-col md:items-center"
      >
        <div class="p-4">
          This extension has been submitted for review. Please approve or reject
          this extension, and when rejecting, provide a reason why.
        </div>
        <div
          class="flex h-full items-center gap-2 border-neutral-700 p-2.5 max-md:w-full max-md:border-t"
        >
          <ElementsButton
            @click="modalOpen.adminReject = true"
            class="h-full max-md:order-last max-md:w-full"
          >
            Reject
          </ElementsButton>
          <ElementsButton
            @click="handleAdminApprove"
            class="h-full max-md:w-full"
          >
            Approve
          </ElementsButton>
        </div>
      </div>

      <div
        class="flex flex-col gap-2 md:flex-row md:items-center md:justify-between"
      >
        <div class="flex items-center gap-2">
          <NuxtLink
            :to="
              data.extension.status == 'approved'
                ? `/browse/${data?.extension.identifier}`
                : ''
            "
          >
            <span
              class="h1"
              :class="
                data.extension.status == 'approved'
                  ? 'text-link !decoration-transparent'
                  : ''
              "
            >
              {{ data.extension.name }}
            </span>
          </NuxtLink>
          <ElementsTextbadge
            v-if="user?.admin && user?.id != data.extension.author.id"
            :label="`${data.extension.author.name}`"
            icon="memory:account"
          />
        </div>
        <div class="flex items-center gap-2 max-md:flex-col">
          <ElementsButton
            v-if="data.extension.status == 'approved'"
            @click="modalOpen.adminReject = true"
            class="max-md:w-full"
          >
            Withdraw approval
          </ElementsButton>
          <template v-if="user?.id == data.extension.author.id">
            <ElementsButton
              v-if="data.extension.status == 'pending'"
              @click="handleSubmit"
              :disabled="submitting"
              class="max-md:w-full"
            >
              Submit for review
            </ElementsButton>
            <ElementsButton
              v-else-if="data.extension.status == 'ready'"
              class="max-md:w-full"
              :disabled="true"
            >
              <div class="flex items-center justify-center gap-1.5">
                <Icon name="pixelarticons:clock" />
                <span>Submit for review</span>
              </div>
            </ElementsButton>
          </template>

          <ElementsButton
            class="max-md:w-full"
            :disabled="
              fieldValidation.extension_name == false ||
              fieldValidation.extension_identifier == false ||
              fieldValidation.extension_summary == false ||
              (form.type != 'extension' && form.type != 'theme') ||
              (form.unlisted != true && form.unlisted != false) ||
              loading
            "
            @click="handleSave"
          >
            Save changes
          </ElementsButton>
        </div>
      </div>

      <ElementsInlinecard
        v-if="
          data.extension.deny_reason &&
          user?.id == data.extension.author.id &&
          data.extension.status != 'ready' &&
          data.extension.status != 'approved'
        "
      >
        Your extension submission was rejected for the following reason(s):
        <span class="text-default-font/60 italic">
          {{ data.extension.deny_reason }}
        </span>
        Please correct this/these issue(s) and re-submit the extension for
        review.
      </ElementsInlinecard>

      <div class="flex flex-col gap-5 xl:flex-row">
        <div
          class="xl:flex-2/3 overflow-hidden rounded-3xl border border-neutral-700"
        >
          <div
            class="h-50 overflow-hidden rounded-3xl bg-cover bg-center xl:h-full"
            :style="`background-image: url(${data.extension.banner.fullres});`"
          >
            <div class="h-50 w-full xl:h-full xl:backdrop-blur-2xl">
              <div
                class="h-50 w-full bg-cover bg-center bg-no-repeat xl:h-full xl:bg-contain"
                :style="`background-image: url(${data.extension.banner.fullres})`"
              >
                <div
                  class="h-50 flex w-full flex-col justify-between xl:h-full"
                >
                  <div
                    class="flex items-center gap-2 rounded-t-3xl border-b border-neutral-700 bg-neutral-950/90 p-4 backdrop-blur-2xl"
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
            </div>
          </div>
        </div>
        <div class="xl:flex-1/3">
          <div class="rounded-3xl border border-neutral-700">
            <div
              class="flex items-center gap-2 border-b border-neutral-700 p-4"
            >
              <Icon name="memory:text-box" :size="28" />
              <span class="h2"> General info </span>
            </div>
            <div class="space-y-4 p-4">
              <ElementsFormInput
                v-model="form.name"
                label="Name"
                description="The display name of your extension, should match the display name you defined in your extension's configuration."
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
                description="The identifier of your extension, should match the identifier you defined in your extension's configuration."
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
                description="A short description of your extension to display alongside your extension listing."
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
              <ElementsFormBinarytoggle
                v-model="form.type"
                label="Type"
                description="Each extension falls under one of these two categories. If your extension moreso focuses on visual changes over functional changes, it's likely a theme."
                :options="[
                  {
                    value: 'extension',
                    icon: 'memory:cube',
                    label: 'Extension',
                  },
                  { value: 'theme', icon: 'memory:image', label: 'Theme' },
                ]"
              />
              <ElementsFormBinarytoggle
                v-model="form.unlisted"
                label="Visibility"
                description="Public extensions are discoverable through the platform, unlisted extension only using their direct link. If your extension is pending review, this setting will immediately apply after approval."
                :options="[
                  {
                    value: false,
                    icon: 'pixelarticons:cloud',
                    label: 'Public',
                  },
                  {
                    value: true,
                    icon: 'pixelarticons:link',
                    label: 'Unlisted',
                  },
                ]"
              />
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
            class="flex grid-cols-2 flex-col overflow-hidden rounded-2xl border border-neutral-700 xl:grid"
          >
            <div class="grid grid-cols-1 bg-neutral-800/40">
              <ElementsFormTextbox
                v-model="form.description"
                :rows="12"
                class="rounded-none border-0 bg-transparent font-mono"
                :placeholder="`(ﾉ*･_･)ﾉ \\\n**markdown** is supported`"
              />
            </div>
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

      <ElementsModal
        v-if="user?.admin"
        :is-open="modalOpen.adminReject"
        :closable="true"
        title="Reject extension"
        @close="modalOpen.adminReject = false"
      >
        <template #default>
          <div class="space-y-4">
            <ElementsFormTextbox
              v-model="adminRejectForm.deny_reason"
              :rows="4"
              placeholder="Why did you reject this extension?"
            />
          </div>
        </template>

        <template #footer>
          <ElementsButton
            label="Cancel"
            class="w-full md:w-auto"
            @click="modalOpen.adminReject = false"
          />
          <ElementsButton
            label="Reject"
            :disabled="loading"
            @click="handleAdminReject"
            type="submit"
            class="order-first w-full md:order-[unset] md:w-auto"
          />
        </template>
      </ElementsModal>
    </template>
  </client-only>
</template>

<script setup lang="ts">
const route = useRoute()
const { user } = useAuth()
const { rules: validationRules } = useFormValidation()

const loading = ref(false)
const submitting = ref(false)
const errors = ref(false)
const data = ref<{ extension: FullExtension }>()
const fieldValidation = ref<Record<string, boolean>>({})
const modalOpen = ref({
  adminReject: false,
})
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
const adminRejectForm = ref<{
  deny_reason: string
}>({
  deny_reason: '',
})

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

onMounted(async () => {
  try {
    data.value = await $fetch(
      user.value?.admin
        ? `/api/user/admin/extensions/${route.params.id}`
        : `/api/user/extensions/${route.params.id}`,
      { method: 'GET', server: false }
    )
  } catch (error) {
    console.error(error)
  }
})

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
        unlisted: newData.extension.unlisted,
        description: newData.extension.description || '',
      }
    }
  },
  { immediate: true }
)

const handleSave = async () => {
  errors.value = false
  loading.value = true

  try {
    await $fetch(
      user.value?.admin
        ? `/api/user/admin/extensions/${route.params.id}`
        : `/api/user/extensions/${route.params.id}`,
      {
        method: 'PATCH',
        body: form.value,
      }
    )
  } catch (error) {
    console.error(error)
    errors.value = true
  } finally {
    loading.value = false
  }
}

const handleSubmit = async () => {
  errors.value = false
  submitting.value = true

  try {
    await $fetch(`/api/user/extensions/${route.params.id}/ready`, {
      method: 'POST',
    })
    if (data.value) data.value.extension.status = 'ready'
  } catch (error) {
    console.error(error)
    errors.value = true
  } finally {
    submitting.value = false
  }
}

const handleAdminApprove = async () => {
  errors.value = false

  try {
    await $fetch(`/api/user/admin/extensions/${route.params.id}/ready`, {
      method: 'POST',
    })
    if (data.value) data.value.extension.status = 'approved'
  } catch (error) {
    console.error(error)
    errors.value = true
  }
}

const handleAdminReject = async () => {
  errors.value = false
  loading.value = true

  try {
    await $fetch(`/api/user/admin/extensions/${route.params.id}/deny`, {
      method: 'POST',
      body: adminRejectForm.value,
    })
    modalOpen.value.adminReject = false
    if (data.value) {
      data.value.extension.status = 'pending'
      data.value.extension.deny_reason = adminRejectForm.value.deny_reason
    }
    adminRejectForm.value.deny_reason = ''
  } catch (error) {
    console.error(error)
    errors.value = true
  } finally {
    loading.value = false
  }
}
</script>
