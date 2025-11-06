export default defineNuxtRouteMiddleware(async (to) => {
  if (import.meta.server) return

  const { isAuthenticated, initializeAuth } = useAuth()

  await initializeAuth()

  if (isAuthenticated.value) {
    return navigateTo('/app', { external: true })
  }
})
