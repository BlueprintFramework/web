export interface Extension {
  id: number
  author: ExtensionAuthor
  type: ExtensionType
  name: string
  identifier: string
  summary: string
  platforms: Record<string, ExtensionPlatforms[]>
  versions: ExtensionVersions[]
  keywords: string[]
  banner: string
  created: string
  stats: {
    panels: number
  }
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
  reviews?: number
  rating?: number | null
}

export interface ExtensionAuthor {
  id: number
  name: string
  website?: string
  support?: string
  created: string
}

export type ExtensionType = 'THEME' | 'EXTENSION'
