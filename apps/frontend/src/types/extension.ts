export interface Extension {
  author: ExtensionAuthor
  banner: string
  created: string
  id: number
  identifier: string
  keywords: string[]
  name: string
  platforms: Record<string, ExtensionPlatforms>
  stats: {
    panels: number
  }
  status: ExtensionStatus
  summary: string
  type: ExtensionType
  unlisted: boolean
  versions: ExtensionVersions[]
  description?: string | null
}

export interface ExtensionVersions {
  name: string
  downloads: number
  created: string
}

export interface ExtensionPlatforms {
  url: string
  price: number
  currency: string
  reviews?: number | null
  rating?: number | null
}

export interface ExtensionAuthor {
  id: number
  name: string
  website?: string | null
  support?: string | null
  created: string
}

export type ExtensionType = 'theme' | 'extension'
export type ExtensionStatus = 'approved' | 'ready' | 'pending'
