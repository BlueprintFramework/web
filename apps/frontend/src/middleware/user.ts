export default defineNuxtRouteMiddleware(async (to) => {
  if (import.meta.server) return

  const { isAuthenticated, isFetched } = useAuth()

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
})
