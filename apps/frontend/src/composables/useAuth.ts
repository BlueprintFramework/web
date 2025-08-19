export const useAuth = () => {
  const authState = useState<AuthState>('auth', () => ({
    user: null,
    isAuthenticated: 'pending',
  }))

  const setUser = (user: FullUser) => {
    authState.value.user = user
    authState.value.isAuthenticated = true
  }

  const login = async (email: string, password: string) => {
    try {
      const data: { user: FullUser } = await $fetch('/api/auth/login', {
        method: 'POST',
        body: {
          email,
          password,
          captcha: null,
        },
      })

      setUser(data.user)

      await navigateTo('/app')
    } catch (error) {
      throw error
    }
  }

  const register = async (email: string, password: string, name: string) => {
    try {
      const data: { user: FullUser } = await $fetch('/api/auth/register', {
        method: 'POST',
        body: {
          name,
          email,
          password,
          captcha: null,
        },
      })

      setUser(data.user)

      await navigateTo('/app')
    } catch (error) {
      throw error
    }
  }

  const logout = async () => {
    try {
      const data = await $fetch('/api/user/logout', {
        method: 'POST',
      })

      authState.value.user = null
      authState.value.isAuthenticated = false

      await navigateTo('/')
    } catch (error) {
      throw error
    }
  }

  const initializeAuth = async () => {
    try {
      const data: { user: FullUser } = await $fetch('/api/user', {
        method: 'GET',
      })

      authState.value.user = data.user
      authState.value.isAuthenticated = true
    } catch {
      authState.value.isAuthenticated = false
    }
  }

  return {
    user: readonly(computed(() => authState.value.user)),
    isAuthenticated: readonly(computed(() => authState.value.isAuthenticated)),

    login,
    register,
    logout,
    initializeAuth,
    setUser,
  }
}
