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

  // isAuthenticated is a bit funky here
  if (!user?.value?.id) {
    return navigateTo('/auth')
  }
})
