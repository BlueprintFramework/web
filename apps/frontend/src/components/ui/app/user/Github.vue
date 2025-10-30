<template>
  <div class="bg-neutral-950 p-4">
    <ElementsInlinecard v-if="error == 'enable-error'" class="mb-4 w-full">
      Uh oh, couldn't enable GitHub oAuth. Please try again later.
    </ElementsInlinecard>
    <ElementsInlinecard v-if="error == 'disable-error'" class="mb-4 w-full">
      Uh oh, couldn't disable GitHub oAuth. Please try again later.
    </ElementsInlinecard>
    <ElementsInlinecard
      v-if="route.query.reason == 'oauth-complete'"
      class="mb-4 w-full"
    >
      Your GitHub account is now connected to Blueprint. If you'd like to
      disconnect it, you can do so below.
    </ElementsInlinecard>
    <div class="flex flex-col items-center py-4">
      <template v-if="user?.github_id">
        <Icon name="pixelarticons:github" :size="32" mode="svg" />
        <br />
        <p class="max-w-100 chrome:-mt-3 my-3 text-center">
          GitHub account
          <b>{{ githubUsername || user?.github_id }}</b> is currently linked to
          Blueprint. You can disconnect it below.
        </p>
        <ElementsButton
          @click="handleDisable"
          color="danger"
          label="Disconnect"
          :disabled="loading"
        />
      </template>
      <template v-else>
        <Icon name="pixelarticons:github" :size="32" mode="svg" />
        <br />
        <p class="max-w-100 my-3 text-center">
          Your account is currently not connected with a GitHub account.
        </p>
        <ElementsButton
          @click="handleEnable"
          label="Connect"
          :disabled="loading"
        />
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
const { user, initializeAuth } = useAuth()
const route = useRoute()

const loading = ref(false)
const error = ref()
const githubUsername = ref('')

const handleEnable = async () => {
  loading.value = true
  error.value = ''

  try {
    let oauth_url = await $fetch<{ redirect_url: string }>(
      `/api/user/github?${Date.now()}`,
      {
        method: 'GET',
      }
    )
    navigateTo(oauth_url?.redirect_url, { external: true })
  } catch {
    loading.value = false
    error.value = 'enable-error'
    throw console.error(
      "Couldn't get github oauth redirect url, try again later."
    )
  }
}

const handleDisable = async () => {
  loading.value = true
  error.value = ''
  try {
    await $fetch(`/api/user/github`, {
      method: 'DELETE',
    })
  } catch {
    loading.value = false
    error.value = 'disable-error'
    throw console.error("Couldn't disable github oauth, try again later.")
  } finally {
    await initializeAuth()
    loading.value = false
  }
}
</script>
