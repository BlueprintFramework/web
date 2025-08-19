export default defineNuxtRouteMiddleware(async (to) => {
  if (import.meta.server) return

  const { isAuthenticated } = useAuth()

  if (isAuthenticated.value === 'pending') {
    await new Promise((resolve) => {
      const unwatch = watch(
        isAuthenticated,
        (newValue) => {
          if (newValue !== 'pending') {
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
