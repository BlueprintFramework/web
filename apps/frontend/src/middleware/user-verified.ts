export default defineNuxtRouteMiddleware(async (to) => {
  if (import.meta.server) return

  const { user, isAuthenticated, isFetched } = useAuth()

  if (!isFetched.value) {
    await new Promise((resolve) => {
      const unwatch = watch(
        isAuthenticated,
        (newValue) => {
          if (isFetched.value) {
            unwatch()
            resolve(undefined)
          }
        },
        { immediate: true }
      )
    })
  }

  if (!isAuthenticated.value) {
    return navigateTo('/auth')
  }

  if (user?.value?.email_pending != null) {
    return navigateTo('/auth/verify')
  }
})
