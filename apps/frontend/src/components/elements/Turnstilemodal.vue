<script setup lang="ts">
const captcha = defineModel<string>({ default: '' })

defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  close: [result: { captcha: string; confirmed: boolean }]
}>()

const turnstileRef = ref()

defineExpose({
  turnstile: computed(() => turnstileRef.value),
})

const handleContinue = () => {
  emit('close', { captcha: captcha.value, confirmed: true })
}

const handleClose = () => {
  emit('close', { captcha: captcha.value, confirmed: false })
}
</script>

<template>
  <ElementsModal
    :is-open="isOpen"
    :closable="true"
    title="Captcha"
    @close="handleClose"
  >
    <template #default>
      <p class="mb-5">
        We need to know if you're human. Please complete the captcha below.
      </p>
      <div class="rounded-2xl border border-neutral-700 bg-neutral-900 py-4">
        <div class="turnstile-wrapper flex min-h-[65px] flex-col items-center">
          <NuxtTurnstile v-model="captcha" ref="turnstileRef" />
        </div>
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Continue"
        class="order-first w-full md:order-[unset] md:w-auto"
        @click="handleContinue"
      />
    </template>
  </ElementsModal>
</template>
