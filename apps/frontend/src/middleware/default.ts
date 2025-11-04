export default defineNuxtRouteMiddleware(async (to) => {
  if (import.meta.server) return

  const { initializeAuth } = useAuth()

  await initializeAuth()
})
