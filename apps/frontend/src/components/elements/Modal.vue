<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition-all duration-250 ease-out"
      enter-from-class="backdrop-blur-lg backdrop-opacity-0"
      enter-to-class="backdrop-blur-lg backdrop-opacity-100"
      leave-active-class="transition-all duration-150 ease-in"
      leave-from-class="backdrop-blur-lg backdrop-opacity-100"
      leave-to-class="backdrop-blur-lg backdrop-opacity-0"
    >
      <div v-if="isOpen" class="z-90 fixed inset-0">
        <div class="fixed inset-0 backdrop-blur-lg" />
      </div>
    </Transition>
    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 scale-95 translate-y-4"
      enter-to-class="opacity-100 scale-100 translate-y-0"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100 scale-100 translate-y-0"
      leave-to-class="opacity-0 scale-95 translate-y-4"
    >
      <div v-if="isOpen" class="z-100 fixed inset-0 transform overflow-y-auto">
        <div
          class="flex min-h-full items-center justify-center p-4"
          @click="handleBackdropClick"
        >
          <div
            ref="modalRef"
            class="relative w-full max-w-lg transform divide-y divide-neutral-700 rounded-3xl border border-neutral-700 bg-neutral-950"
            role="dialog"
            :aria-labelledby="titleId"
            :aria-describedby="descriptionId"
            aria-modal="true"
            @click.stop
          >
            <div
              v-if="title || $slots.header"
              class="flex items-center justify-between p-4"
            >
              <div v-if="$slots.header">
                <slot name="header" />
              </div>
              <h2 v-else-if="title" :id="titleId" class="text-xl font-semibold">
                {{ title }}
              </h2>

              <button
                v-if="closable"
                type="button"
                class="hover:text-brand-50 rounded-full border border-neutral-700 bg-neutral-900 p-1 transition-colors hover:bg-neutral-800"
                @click="closeModal"
              >
                <span class="sr-only">Close modal</span>
                <Icon name="pixelarticons:close" :size="24" mode="svg" />
              </button>
            </div>

            <div class="p-4">
              <slot />
            </div>

            <div
              v-if="$slots.footer"
              class="flex flex-row items-center justify-end gap-2 p-4"
            >
              <slot name="footer" />
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
interface Props {
  isOpen: boolean
  title?: string
  closable?: boolean
  closeOnBackdrop?: boolean
}

interface Emits {
  close: []
}

const props = withDefaults(defineProps<Props>(), {
  closable: true,
  closeOnBackdrop: true,
})

const emit = defineEmits<Emits>()

const titleId = `modal-title-${Math.random().toString(36).substring(2, 9)}`
const descriptionId = `modal-description-${Math.random().toString(36).substring(2, 9)}`

const modalRef = ref<HTMLElement>()

const handleBackdropClick = (event: MouseEvent) => {
  if (props.closeOnBackdrop && event.target === event.currentTarget) {
    closeModal()
  }
}

const closeModal = () => {
  emit('close')
}

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.closable) {
    closeModal()
  }
}

const focusableElements = computed(() => {
  if (!modalRef.value) return []

  const selector =
    'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
  return Array.from(modalRef.value.querySelectorAll(selector)) as HTMLElement[]
})

const firstFocusableElement = computed(() => focusableElements.value[0])
const lastFocusableElement = computed(
  () => focusableElements.value[focusableElements.value.length - 1]
)

const handleTabKey = (event: KeyboardEvent) => {
  if (event.key !== 'Tab') return

  if (focusableElements.value.length === 0) {
    event.preventDefault()
    return
  }

  if (event.shiftKey) {
    if (document.activeElement === firstFocusableElement.value) {
      event.preventDefault()
      lastFocusableElement.value?.focus()
    }
  } else {
    if (document.activeElement === lastFocusableElement.value) {
      event.preventDefault()
      firstFocusableElement.value?.focus()
    }
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
  document.addEventListener('keydown', handleTabKey)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
  document.removeEventListener('keydown', handleTabKey)
})

watch(
  () => props.isOpen,
  (isOpen) => {
    if (isOpen) {
      nextTick(() => {
        firstFocusableElement.value?.focus() || modalRef.value?.focus()
      })
      document.body.style.overflow = 'hidden'
    } else {
      document.body.style.overflow = ''
    }
  }
)

onUnmounted(() => {
  document.body.style.overflow = ''
})
</script>
