export default defineNuxtRouteMiddleware(async (to) => {
  if (import.meta.server) return

  const { user, isAuthenticated, initializeAuth } = useAuth()

  await initializeAuth()

  if (!isAuthenticated.value) {
    return navigateTo('/auth', { external: true })
  }

  if (user?.value?.email_pending != null) {
    return navigateTo('/auth/verify', { external: true })
  }
})
