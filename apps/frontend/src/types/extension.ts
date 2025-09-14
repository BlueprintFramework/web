export interface Extension {
  author: User
  banner: ExtensionBanner
  created: string
  id: number
  identifier: string
  keywords: string[]
  name: string
  platforms: Record<string, ExtensionFullPlatform>
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

export interface ExtensionPlatform {
  currency: ExtensionPlatformCurrency
  price: number
  url: string
}

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
export type ExtensionPlatformCurrency = 'USD' | 'EUR' | 'GBP'
