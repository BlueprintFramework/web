<template>
  <ElementsModal
    :is-open="isOpen"
    :closable="true"
    title="Insert image"
    @close="isOpen = false"
  >
    <template #default>
      <div
        v-if="extension.status != 'approved'"
        class="flex flex-col items-center py-4"
      >
        <div class="max-w-95 flex flex-col items-center gap-2">
          <Icon name="pixelarticons:image-delete" :size="32" />
          <p class="text-lg font-bold">Hold on!</p>
          <p class="text-default-font/60 text-center">
            To prevent abuse, your extension has to be approved to upload new
            images.
          </p>
        </div>
      </div>

      <template v-else>
        <div v-if="loading" class="grid grid-cols-1 gap-2 md:grid-cols-2">
          <div
            v-for="i in imageCount"
            class="aspect-video overflow-hidden rounded-2xl border border-neutral-700"
          >
            <div class="h-full w-full animate-pulse bg-neutral-800/70" />
          </div>
        </div>

        <template v-else>
          <div
            v-if="!images?.extension_images[0]"
            class="flex flex-col items-center py-4"
          >
            <div class="max-w-95 flex flex-col items-center gap-2">
              <Icon name="pixelarticons:image-broken" :size="32" />
              <p class="text-lg font-bold">No images yet</p>
              <p class="text-default-font/60 text-center">
                Once you upload images, they'll show up here. You can upload up
                to 25 images per extension.
              </p>
            </div>
          </div>
          <div v-else class="grid grid-cols-1 gap-2 md:grid-cols-2">
            <div
              v-for="image in images.extension_images"
              class="group aspect-video cursor-pointer overflow-hidden rounded-2xl border border-neutral-700 bg-cover bg-center p-2 transition-colors hover:border-neutral-500"
              :style="`background-image: url(${image.url});`"
            >
              <div class="flex justify-between gap-2">
                <ElementsButtonSmall
                  class="min-w-auto hover:text-brand-50 !rounded-lg py-1 !transition-all hover:bg-neutral-800 group-hover:!rounded-sm group-hover:py-2"
                  @click="handleInsert(image)"
                >
                  <Icon name="pixelarticons:plus" mode="svg" />
                </ElementsButtonSmall>
                <ElementsButtonSmall
                  color="danger"
                  class="min-w-auto !rounded-lg rounded-2xl py-1 !transition-all group-hover:!rounded-sm group-hover:py-2"
                  @click="handleDelete(image)"
                >
                  <Icon name="pixelarticons:trash" mode="svg" />
                </ElementsButtonSmall>
              </div>
            </div>
          </div>
        </template>
      </template>
    </template>

    <template #footer>
      <ElementsButton
        v-if="extension.status == 'approved'"
        class="flex w-full flex-col items-center md:w-auto"
        :disabled="loading"
        @click="uploadInput?.click()"
      >
        <div class="flex items-center gap-1.5">
          <Icon name="pixelarticons:upload" />
          <span>Upload image</span>
        </div>
      </ElementsButton>
      <ElementsButton
        label="Cancel"
        class="order-first w-full md:order-[unset] md:w-auto"
        @click="isOpen = false"
      />
    </template>
  </ElementsModal>

  <input
    ref="uploadInput"
    type="file"
    accept="image/*"
    class="hidden"
    @change="handleUpload"
  />
</template>

<script setup lang="ts">
const { user } = useAuth()

const props = defineProps<{
  extension: FullExtension
}>()

const emit = defineEmits<{
  insert: [url: ExtensionImage['url']]
}>()

const uploadInput = useTemplateRef('uploadInput')

const isOpen = ref(true)
const loading = ref(false)
const images = ref<{ extension_images: ExtensionImages }>()
const imageCount = ref(2)
const basePath = ref(`/api/user/extensions/${props.extension.identifier}`)

if (user.value?.admin) {
  basePath.value = `/api/user/admin/extensions/${props.extension.identifier}`
}

const fetchImages = async () => {
  try {
    images.value = await $fetch(`${basePath.value}/images`, {
      method: 'GET',
    })
  } catch (error) {
    console.error('An error occurred fetching images: ' + error)
  }
}

const handleOpen = async () => {
  isOpen.value = true
  if (props.extension.status == 'approved') {
    loading.value = true
    await fetchImages()
    loading.value = false
  }
}

const handleUpload = async (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return

  imageCount.value = (images.value?.extension_images.length || 0) + 1
  loading.value = true

  try {
    await $fetch(`${basePath.value}/images`, {
      method: 'POST',
      body: file,
    })
    await fetchImages()
  } catch (error) {
    console.error(error)
  } finally {
    loading.value = false
  }
}

const handleInsert = async (image: ExtensionImage) => {
  isOpen.value = false
  emit('insert', image.url)
}

const handleDelete = async (image: ExtensionImage) => {
  if (!image) return

  imageCount.value = (images.value?.extension_images.length || 0) - 1
  loading.value = true

  try {
    await $fetch(`${basePath.value}/images/${image.id}`, { method: 'DELETE' })
  } catch (error) {
    console.error(error)
  } finally {
    await fetchImages()
    loading.value = false
  }
}

defineExpose({
  isOpen,
  handleOpen,
})
</script>
