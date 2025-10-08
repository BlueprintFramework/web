export interface Extension {
  author: User
  banner: ExtensionBanner
  created: string
  id: number
  identifier: string
  keywords: string[]
  name: string
  platforms: ExtensionFullPlatforms
  stats: ExtensionStats
  summary: string
  type: ExtensionType
  versions: ExtensionVersion[]
  description?: string | null
}

export interface FullExtension extends Extension {
  status: ExtensionStatus
  unlisted: boolean
  deny_reason?: string | null
}

export interface ExtensionVersion {
  name: string
  downloads: number
  created: string
}

export type ExtensionPlatforms = Record<string, ExtensionPlatform>
export type ExtensionPlatformCurrency = 'USD' | 'EUR' | 'GBP'
export interface ExtensionPlatform {
  currency: ExtensionPlatformCurrency
  price: number
  url: string
}

export type ExtensionFullPlatforms = Record<string, ExtensionFullPlatform>
export interface ExtensionFullPlatform extends ExtensionPlatform {
  reviews?: number | null
  rating?: number | null
}

export interface ExtensionBanner {
  fullres: string
  lowres: string
}

export interface ExtensionStats {
  panels: number
}

export type ExtensionType = 'theme' | 'extension'
export type ExtensionStatus = 'approved' | 'ready' | 'pending'
