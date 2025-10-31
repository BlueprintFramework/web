<template>
  <div class="flex flex-col items-center justify-center bg-neutral-950 p-4">
    <ElementsInlinecard v-if="error == 'enable-error'" class="mb-4">
      Uh oh, couldn't initialize 2FA. Please try again later.
    </ElementsInlinecard>
    <div class="flex flex-col items-center py-4">
      <template v-if="user?.totp_enabled">
        <Icon name="pixelarticons:lock" :size="32" mode="svg" />
        <br />
        <p class="max-w-100 chrome:-mt-3 my-3 text-center">
          You've enabled two factor authentication! You can disable it below.
        </p>
        <ElementsButton
          @click="modalOpen.disable_2fa = true"
          color="danger"
          label="Disable 2FA"
          :disabled="loading"
        />
      </template>
      <template v-else>
        <Icon name="pixelarticons:lock-open" :size="32" mode="svg" />
        <br />
        <p class="max-w-100 my-3 text-center">
          You haven't enabled two factor authentication yet! To keep your
          account secure, set it up below.
        </p>
        <ElementsButton
          @click="handleEnable"
          label="Enable 2FA"
          :disabled="loading"
        />
      </template>
    </div>
  </div>

  <ElementsModal
    :is-open="modalOpen.enable_2fa"
    :closable="false"
    title="Set up 2FA"
    @close="modalOpen.enable_2fa = false"
  >
    <template #default>
      <div
        class="flex flex-col gap-4 overflow-hidden md:flex-row md:items-center"
      >
        <div>
          <div class="inline-block rounded-2xl bg-white p-2">
            <ElementsQrcode
              v-if="enableOtpData?.otp_url"
              :value="enableOtpData.otp_url"
              :size="160"
            />
          </div>
        </div>
        <div class="space-y-2">
          <p>
            <b>Scan the QR code with your two factor authentication app</b> or
            use the credential below.
          </p>
          <div class="rounded-2xl border border-neutral-700 p-2 font-mono">
            <span class="wrap-anywhere text-wrap">
              {{ enableOtpData?.secret }}
            </span>
          </div>
        </div>
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Cancel"
        color="danger"
        class="w-full md:w-auto"
        @click="modalOpen.enable_2fa = false"
      />
      <ElementsButton
        label="Continue"
        class="order-first w-full md:order-[unset] md:w-auto"
        @click="
          () => {
            modalOpen.enable_2fa = false
            modalOpen.confirm_2fa = true
          }
        "
      />
    </template>
  </ElementsModal>

  <ElementsModal
    :is-open="modalOpen.confirm_2fa"
    :closable="false"
    title="Set up 2FA"
    @close="
      () => {
        modalOpen.confirm_2fa = false
        modalOpen.enable_2fa = true
      }
    "
  >
    <template #default>
      <div class="space-y-4">
        <ElementsInlinecard v-if="error == 'confirm-error'">
          An error occurred. Make sure you used the correct password and 2FA
          code.
        </ElementsInlinecard>

        <p>
          Provide the 2FA code generated earlier and your account password to
          apply your changes.
        </p>

        <ElementsFormInput
          v-model="form.code"
          name="code"
          type="text"
          :rules="[validationRules.required(), validationRules.code()]"
          :required="true"
          leading-icon="memory:shield"
          autocomplete="one-time-code"
          placeholder="2FA code"
          :disabled="loading"
          @validate="
            (isValid: boolean) => handleFieldValidation('code', isValid)
          "
        />

        <ElementsFormInput
          v-model="form.password"
          name="password"
          type="password"
          :rules="[validationRules.required()]"
          :required="true"
          leading-icon="memory:key"
          autocomplete="current-password"
          placeholder="Password"
          :disabled="loading"
          @validate="
            (isValid: boolean) => handleFieldValidation('password', isValid)
          "
        />
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Back"
        class="w-full md:w-auto"
        @click="
          () => {
            modalOpen.confirm_2fa = false
            modalOpen.enable_2fa = true
          }
        "
      />
      <ElementsButton
        :disabled="
          fieldValidation.code == false ||
          fieldValidation.password == false ||
          loading
        "
        label="Enable 2FA"
        class="order-first w-full md:order-[unset] md:w-auto"
        @click="handleConfirm"
      />
    </template>
  </ElementsModal>

  <ElementsModal
    :is-open="modalOpen.recovery_codes"
    title="Recovery codes"
    @close="modalOpen.recovery_codes = false"
  >
    <template #default>
      <div class="space-y-4">
        <p>
          2FA has been enabled on your account. Below is a list of recovery
          codes in case you lose your authenticator, save them somewhere secure!
        </p>

        <div class="rounded-2xl border border-neutral-700 p-2 font-mono">
          <span class="text-wrap">
            {{ confirmOtpData?.recovery_codes.join(' ') }}
          </span>
        </div>
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Close"
        class="w-full md:w-auto"
        @click="modalOpen.recovery_codes = false"
      />
      <ElementsButton
        label="Download codes"
        class="order-first w-full md:order-[unset] md:w-auto"
        @click="handleDownloadRecoveryCodes"
      />
    </template>
  </ElementsModal>

  <ElementsModal
    :is-open="modalOpen.disable_2fa"
    title="Disable 2FA"
    @close="modalOpen.disable_2fa = false"
  >
    <template #default>
      <div class="space-y-4">
        <ElementsInlinecard v-if="error == 'disable-error'">
          An error occurred. Make sure you used the correct password and 2FA
          code.
        </ElementsInlinecard>

        <p>
          Disabling two factor authentication can pose additional risks to your
          account. Consider leaving it enabled.
        </p>

        <ElementsFormInput
          v-model="form.code"
          name="code"
          type="text"
          :rules="[validationRules.required(), validationRules.code()]"
          :required="true"
          leading-icon="memory:shield"
          autocomplete="one-time-code"
          placeholder="2FA code"
          :disabled="loading"
          @validate="
            (isValid: boolean) => handleFieldValidation('code', isValid)
          "
        />

        <ElementsFormInput
          v-model="form.password"
          name="password"
          type="password"
          :rules="[validationRules.required()]"
          :required="true"
          leading-icon="memory:key"
          autocomplete="current-password"
          placeholder="Password"
          :disabled="loading"
          @validate="
            (isValid: boolean) => handleFieldValidation('password', isValid)
          "
        />
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Cancel"
        class="w-full md:w-auto"
        @click="modalOpen.disable_2fa = false"
      />
      <ElementsButton
        :disabled="
          fieldValidation.code == false ||
          fieldValidation.password == false ||
          loading
        "
        label="Disable 2FA"
        color="danger"
        class="order-first w-full md:order-[unset] md:w-auto"
        @click="handleDisable"
      />
    </template>
  </ElementsModal>
</template>

<script setup lang="ts">
import { saveAs } from 'file-saver'

const { user, initializeAuth } = useAuth()
const { rules: validationRules } = useFormValidation()

const loading = ref(false)
const error = ref()
const fieldValidation = ref<Record<string, boolean>>({})
const modalOpen = ref({
  enable_2fa: false,
  confirm_2fa: false,
  recovery_codes: false,
  disable_2fa: false,
})
const form = ref({
  password: '',
  code: '',
})
const enableOtpData = ref<{
  otp_url: string
  secret: string
}>()
const confirmOtpData = ref<{
  recovery_codes: string[]
}>()

const handleFieldValidation = (field: string, isValid: boolean) => {
  fieldValidation.value[field] = isValid
}

const handleEnable = async () => {
  loading.value = true
  error.value = ''
  try {
    enableOtpData.value = await $fetch(`/api/user/two-factor`, {
      method: 'GET',
    })
  } catch {
    error.value = 'enable-error'
    throw console.error("Couldn't get 2fa data, try again later.")
  } finally {
    loading.value = false
    modalOpen.value.enable_2fa = true
  }
}

const handleConfirm = async () => {
  loading.value = true
  error.value = ''
  try {
    confirmOtpData.value = await $fetch(`/api/user/two-factor`, {
      method: 'POST',
      body: form.value,
    })
  } catch {
    loading.value = false
    error.value = 'confirm-error'
    throw console.error("Couldn't enable 2fa, try again later.")
  } finally {
    await initializeAuth()
    loading.value = false
    modalOpen.value.confirm_2fa = false
    modalOpen.value.recovery_codes = true
    form.value = {
      password: '',
      code: '',
    }
  }
}

const handleDisable = async () => {
  loading.value = true
  error.value = ''
  try {
    confirmOtpData.value = await $fetch(`/api/user/two-factor`, {
      method: 'DELETE',
      body: form.value,
    })
  } catch {
    loading.value = false
    error.value = 'disable-error'
    throw console.error("Couldn't disable 2fa, try again later.")
  } finally {
    await initializeAuth()
    loading.value = false
    modalOpen.value.disable_2fa = false
    form.value = {
      password: '',
      code: '',
    }
  }
}

const handleDownloadRecoveryCodes = async () => {
  var blob = new Blob(
    [
      `2FA recovery codes for blueprint.zip, save them somewhere secure.\n\n${confirmOtpData.value?.recovery_codes.join('\n')}`,
    ],
    { type: 'text/plain;charset=utf-8' }
  )
  saveAs(blob, 'blueprint-recovery-codes.txt')
}
</script>
