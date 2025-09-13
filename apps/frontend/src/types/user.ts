export interface AuthState {
  user?: FullUser | null
  isAuthenticated: boolean
  isFetched: boolean
  checkpoint: {
    authType?: AuthType
    token?: string
  }
}

export interface FullUser {
  admin: boolean
  created: string
  email: string
  id: number
  name: string
  totp_enabled: boolean
  email_pending?: string | null
  pronouns?: string | null
  support?: string | null
  suspended: boolean
}

export interface User {
  admin: boolean
  created: string
  id: number
  name: string
  pronouns?: string | null
  support?: string | null
  suspended: boolean
}

export interface UserSessions {
  total: number
  per_page: number
  page: number
  data: UserSession[]
}

export interface UserSession {
  created: string
  id: number
  ip: string
  is_using: boolean
  last_used: string
  user_agent: string
}

export type AuthType = 'completed' | 'two_factor_required' | null
