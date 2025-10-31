<template>
  <div class="flex flex-col items-center justify-center bg-neutral-950 p-4">
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
        <NuxtImg
          :src="githubData?.avatar_url"
          :width="32"
          :height="32"
          class="rounded-sm"
        />
        <br />
        <p class="max-w-100 chrome:-mt-3 my-3 text-center">
          <span>Your GitHub account (</span>
          <NuxtLink :to="githubData?.html_url" class="text-link">
            <b> {{ githubData?.login || user?.github_id }} </b>
          </NuxtLink>
          <span>) is linked to your account and can be used for sign-in.</span>
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
          Your GitHub account is currently not linked to Blueprint.
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

interface GithubUser {
  login: string
  avatar_url: string
  url: string
  html_url: string
}

const loading = ref(false)
const error = ref()

const { data: githubData } = useLazyAsyncData('github-user', async () => {
  if (!user.value?.github_id) return null
  return await $fetch<GithubUser>(
    `https://api.github.com/user/${user.value.github_id}`
  )
})

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
