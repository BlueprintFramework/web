<template>
  <template v-if="data?.extension">
    <ElementsInlinecard v-if="errors">
      An unknown error occurred. Open your browser console for more information.
    </ElementsInlinecard>
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
          @click="modalOpen.platforms = true"
          class="w-full md:w-auto"
        >
          Platforms
        </ElementsButton>

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

        <ElementsButton
          v-if="data.extension.status != 'approved'"
          class="max-md:w-full"
          color="danger"
          :disabled="loading"
          @click="handleOpenDelete"
        >
          Delete draft
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
      Please correct this/these issue(s) and re-submit the extension for review.
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
              <div class="h-50 flex w-full flex-col justify-between xl:h-full">
                <div
                  class="flex items-center gap-2 rounded-t-3xl border-b border-neutral-700 bg-neutral-950/90 p-4 backdrop-blur-2xl"
                >
                  <Icon name="memory:image" :size="28" />
                  <span class="h2"> Banner </span>
                </div>
                <div class="flex w-full flex-col items-end justify-end p-4">
                  <div>
                    <input
                      ref="bannerInput"
                      type="file"
                      accept="image/jpeg"
                      class="hidden"
                      @change="handleBannerUpload"
                    />
                    <ElementsButton
                      @click="modalOpen.uploadBanner = true"
                      :disabled="uploading"
                    >
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
          <div class="flex items-center gap-2 border-b border-neutral-700 p-4">
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
              :disabled="data.extension.status == 'approved' || loading"
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
              description="Public extensions are discoverable through the platform, unlisted only available to users with access. If your extension is pending review, this setting will immediately apply after approval."
              :options="[
                {
                  value: false,
                  icon: 'pixelarticons:cloud',
                  label: 'Public',
                },
                {
                  value: true,
                  icon: 'pixelarticons:hidden',
                  label: 'Unlisted',
                },
              ]"
            />
          </div>
        </div>
      </div>
    </div>

    <div class="overflow-hidden rounded-3xl border border-neutral-700">
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
      <div
        class="bg-stripes flex grid-cols-2 flex-col divide-y divide-neutral-700 overflow-hidden rounded-2xl xl:grid xl:divide-x xl:divide-y-0"
      >
        <div>
          <div
            class="border-neutral-700 bg-neutral-950 p-4 xl:-mb-[2px] xl:border-b"
          >
            <ElementsFormTextboxToolbar
              :textarea-ref="descriptionBox"
              :on-image="imagesModal?.handleOpen"
            />
            <ElementsFormTextbox
              v-model="form.description"
              ref="descriptionBox"
              class="font-mono"
              :supports-images="true"
              :rows="10"
              :placeholder="`\`[  > <]\` \\\n**markdown** is supported`"
              :richtext="true"
            />
          </div>
        </div>
        <div>
          <div
            class="border-neutral-700 bg-neutral-950 p-4 xl:-mb-[2px] xl:border-b"
          >
            <template v-if="!form.description || form.description == ''">
              <!-- prettier-ignore -->
              <pre><ProseCode>[  > <]</ProseCode></pre>
              <p><b>markdown</b> is supported</p>
            </template>
            <client-only v-else>
              <MDC
                class="prose-content"
                :value="form.description"
                :parser-options="{
                  rehype: { options: { allowDangerousHtml: false } },
                }"
              />
            </client-only>
          </div>
        </div>
      </div>
    </div>

    <ElementsModal
      :is-open="modalOpen.uploadBanner"
      :closable="true"
      title="Banner guidelines"
      @close="modalOpen.uploadBanner = false"
    >
      <template #default>
        <div class="space-y-4">
          <ElementsInlinecard>
            If your extension fails to meet the quality guidelines for banners,
            we may replace the banner or unlist/reject your extension.
          </ElementsInlinecard>
          <div class="space-y-2">
            <li class="ms-4">
              <p>
                <b>AI-generated graphics are not allowed.</b> Avoid using any
                AI-generated content in your extension graphics.
              </p>
            </li>
            <li class="ms-4">
              <p>
                <b>No plain screenshots.</b> Plain screenshots often add
                additional confusion to buyers and worsen the overall
                experience. Add screenshots to the extension's description
                instead.
              </p>
            </li>
            <li class="ms-4">
              <p>
                <b>Anything is better than nothing.</b> Whether you use
                Microsoft Paint, Canva or Photoshop, just try to make a quick
                simple banner. Extensions with higher quality banners tend to
                get more attention.
              </p>
            </li>
            <li class="ms-4">
              <p>
                Follow our
                <NuxtLink to="/legal/terms" class="text-link"
                  >Terms of Service</NuxtLink
                >
                and
                <NuxtLink to="/legal/conduct" class="text-link"
                  >Code of Conduct</NuxtLink
                >.
              </p>
            </li>
          </div>
        </div>
      </template>

      <template #footer>
        <ElementsButton
          label="Cancel"
          class="w-full md:w-auto"
          @click="modalOpen.uploadBanner = false"
        />
        <ElementsButton
          label="Accept and upload"
          class="flex w-full flex-col items-center md:w-auto"
          :disabled="loading"
          @click="bannerInput?.click()"
        />
      </template>
    </ElementsModal>

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

    <ElementsModal
      v-if="data.extension.status != 'approved'"
      :is-open="modalOpen.delete"
      :closable="true"
      title="Delete extension draft"
      @close="modalOpen.delete = false"
    >
      <template #default>
        <div class="flex flex-col items-center py-4">
          <div class="max-w-95 flex flex-col items-center gap-2">
            <Icon name="pixelarticons:trash" :size="32" />
            <p class="text-lg font-bold">Are you sure?</p>
            <p class="text-default-font/60 text-center">
              Deleting extension drafts is permanent, it cannot be undone.
            </p>
          </div>
        </div>
      </template>

      <template #footer>
        <ElementsButton
          label="Cancel"
          class="w-full md:w-auto"
          @click="modalOpen.delete = false"
        />
        <ElementsButton
          :label="`Delete${deleteTimeout != 0 ? ' (' + deleteTimeout + ')' : ''}`"
          color="danger"
          :disabled="loading || deleteTimeout != 0"
          @click="handleDelete"
          type="submit"
          class="order-first w-full md:order-[unset] md:w-auto"
        />
      </template>
    </ElementsModal>

    <UiAppExtensionsPlatformsmodal
      :is-open="modalOpen.platforms"
      :platforms="form.platforms"
      @close="modalOpen.platforms = false"
      @save="handlePlatformsSave"
    />

    <UiAppExtensionsImagesmodal
      ref="imagesModal"
      :extension="data.extension"
      @insert="
        (url: ExtensionImage['url']) => {
          descriptionBox?.insertMarkdown('![', `](${url})`)
        }
      "
    />
  </template>
</template>

<script setup lang="ts">
const route = useRoute()
const { user } = useAuth()
const { rules: validationRules } = useFormValidation()

const bannerInput = useTemplateRef('bannerInput')
const descriptionBox = useTemplateRef('descriptionBox')
const imagesModal = useTemplateRef('imagesModal')

const basePath = ref(`/api/user/extensions/${route.params.id}`)
const loading = ref(false)
const submitting = ref(false)
const uploading = ref(false)
const errors = ref(false)
const deleteTimeout = ref(10)
const deleteInterval = ref()
const data = ref<{ extension: FullExtension }>()
const fieldValidation = ref<Record<string, boolean>>({})
const modalOpen = ref({
  adminReject: false,
  platforms: false,
  delete: false,
  uploadBanner: false,
})
const form = ref<{
  identifier: string
  name: string
  platforms: ExtensionPlatformUrls
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

if (user.value?.admin) {
  basePath.value = `/api/user/admin/extensions/${route.params.id}`
}

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

onMounted(async () => {
  try {
    data.value = await $fetch(`${basePath.value}`, {
      method: 'GET',
      server: false,
    })
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
        platforms: form.value.platforms,
        summary: newData.extension.summary || '',
        type: newData.extension.type || 'extension',
        unlisted: newData.extension.unlisted,
        description: newData.extension.description || '',
      }

      const urls: ExtensionPlatformUrls = {}
      for (const key in newData.extension.platforms) {
        if (
          Object.prototype.hasOwnProperty.call(newData.extension.platforms, key)
        ) {
          urls[key] = newData.extension.platforms[key]?.url || ''
        }
      }

      form.value.platforms = urls
    }
  },
  { immediate: true }
)

const handleSave = async () => {
  errors.value = false
  loading.value = true

  try {
    await $fetch(`${basePath.value}`, {
      method: 'PATCH',
      body: form.value,
    })
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

const handleBannerUpload = async (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return

  modalOpen.value.uploadBanner = false
  uploading.value = true

  try {
    await $fetch(`${basePath.value}/banner`, {
      method: 'POST',
      body: file,
    })
    const newData = await $fetch<{ extension: FullExtension }>(
      `${basePath.value}`,
      {
        method: 'GET',
      }
    )
    if (data.value && newData) {
      data.value.extension.banner.fullres = newData.extension.banner.fullres
      data.value.extension.banner.lowres = newData.extension.banner.lowres
    }
  } catch (error) {
    console.error(error)
  } finally {
    uploading.value = false
  }
}

const handleOpenDelete = async () => {
  deleteTimeout.value = 10
  modalOpen.value.delete = true

  if (deleteInterval.value) clearInterval(deleteInterval.value)

  deleteInterval.value = setInterval(() => {
    if (deleteTimeout.value === 0) {
      return clearInterval(deleteInterval.value)
    }
    deleteTimeout.value--
  }, 1000)
}

const handleDelete = async () => {
  errors.value = false
  loading.value = true

  try {
    await $fetch(`${basePath.value}`, {
      method: 'DELETE',
    })
  } catch (error) {
    console.error(error)
    errors.value = true
  } finally {
    await navigateTo('/app/extensions')
    loading.value = false
  }
}

const handlePlatformsSave = (platforms: ExtensionPlatformUrls) => {
  form.value.platforms = platforms
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
