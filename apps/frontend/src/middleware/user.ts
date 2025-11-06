export default defineNuxtRouteMiddleware(async (to) => {
  if (import.meta.server) return

  const { user, isAuthenticated, isFetched, initializeAuth } = useAuth()

  await initializeAuth()

  if (!user?.value?.id) {
    return navigateTo('/auth', { external: true })
  }
})
