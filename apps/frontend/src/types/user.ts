export interface AuthState {
  user?: FullUser | null
  token?: string | null
  isAuthenticated: boolean
}

export interface FullUser {
  admin: boolean
  created: string
  email: string
  id: number
  name: string
  email_pending?: string | null
  support?: string | null
}

export interface User {
  admin: boolean
  created: string
  id: number
  name: string
  support?: string | null
}

export interface UserSession {
  created: string
  id: number
  ip: string
  is_using: boolean
  last_used: string
  user_agent: string
}
