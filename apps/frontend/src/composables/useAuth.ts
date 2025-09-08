export const useAuth = () => {
  const authState = useState<AuthState>('auth', () => ({
    user: null,
    isAuthenticated: false,
    isFetched: false,
    checkpoint: {
      authType: undefined,
      token: undefined,
    },
  }))

  const setUser = (user: FullUser) => {
    authState.value.user = user
    authState.value.isFetched = true
    authState.value.isAuthenticated = true
    authState.value.checkpoint.authType = undefined
  }

  const login = async (user: string, password: string) => {
    try {
      const data: {
        user?: FullUser | null
        type?: AuthType
        token?: string
        errors?: ApiError
      } = await $fetch('/api/auth/login', {
        method: 'POST',
        body: {
          user,
          password,
          captcha: null,
        },
      })
      if (data?.user) {
        if (data.type == 'completed') {
          // [INFO] user completed authentication
          authState.value.checkpoint.authType = 'completed'
          setUser(data.user)

          await navigateTo('/app')
        } else {
          // [INFO] send user to 2fa checkpoint
          authState.value.checkpoint.authType = 'two_factor_required'
        }
      } else {
        throw data
      }
    } catch (error: any) {
      if (error.response._data.errors) {
        throw error.response._data.errors
      } else if (error.errors) {
        throw error.errors
      }
    }
  }

  const checkpoint = async (code: number | string) => {
    try {
      const data: { user?: FullUser | null; errors?: ApiError } = await $fetch(
        '/api/auth/checkpoint',
        {
          method: 'POST',
          body: {
            code,
            token: authState.value.checkpoint.token,
          },
        }
      )
      if (data?.user) {
        // [INFO] user completed authentication
        authState.value.checkpoint.authType = 'completed'
        authState.value.checkpoint.token = undefined
        setUser(data.user)

        await navigateTo('/app')
      } else {
        throw data
      }
    } catch (error: any) {
      if (error.response._data.errors) {
        throw error.response._data.errors
      } else if (error.errors) {
        throw error.errors
      }
    }
  }

  const register = async (email: string, password: string, name: string) => {
    try {
      const data: { user?: FullUser; errors?: ApiError } = await $fetch(
        '/api/auth/register',
        {
          method: 'POST',
          body: {
            name,
            email,
            password,
            captcha: null,
          },
        }
      )

      if (data.user) {
        authState.value.checkpoint.authType = 'completed'
        setUser(data.user)
        await navigateTo('/app')
      } else {
        throw data
      }
    } catch (error: any) {
      if (error.response._data.errors) {
        throw error.response._data.errors
      } else if (error.errors) {
        throw error.errors
      }
    }
  }

  const logout = async () => {
    try {
      await $fetch('/api/user/logout', {
        method: 'POST',
      })

      authState.value.user = null
      authState.value.isAuthenticated = false
      authState.value.checkpoint.authType = undefined

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

      setUser(data.user)
    } catch {
      authState.value.isAuthenticated = false
      authState.value.isFetched = true
    }
  }

  return {
    user: readonly(computed(() => authState.value.user)),
    isAuthenticated: readonly(computed(() => authState.value.isAuthenticated)),
    isFetched: readonly(computed(() => authState.value.isFetched)),
    checkpointData: readonly(computed(() => authState.value.checkpoint)),

    login,
    checkpoint,
    register,
    logout,
    initializeAuth,
    setUser,
  }
}
