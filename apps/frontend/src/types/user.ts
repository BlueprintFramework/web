export interface AuthState {
  user?: FullUser | null
  isAuthenticated: boolean
  isFetched: boolean
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

export interface UserSessions {
  sessions: {
    total: number
    per_page: number
    page: number
    data: UserSession[]
  }
}

export interface UserSession {
  created: string
  id: number
  ip: string
  is_using: boolean
  last_used: string
  user_agent: string
}
