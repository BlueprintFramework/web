export interface AuthState {
  user?: FullUser | null
  isAuthenticated: boolean
  isFetched: boolean
  checkpoint: {
    authType?: AuthType
  }
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

export interface FullUser extends User {
  email: string
  totp_enabled: boolean
  email_pending?: string | null
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
