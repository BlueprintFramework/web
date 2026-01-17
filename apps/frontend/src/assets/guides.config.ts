export interface CategoryConfig {
  icon: string
  label?: string
  order?: number
  thumbnail?: string
}

export const guidesCategories: Record<string, CategoryConfig> = {
  admin: {
    icon: 'memory:terminal',
    label: 'System Administration',
    order: 1,
  },
  dev: {
    icon: 'memory:cube',
    label: 'Extension Development',
    order: 2,
  },
  community: {
    icon: 'memory:comment-user',
    label: 'Community Guides',
    order: 3,
  },
  extra: {
    icon: 'memory:dagger',
    label: 'Extra Guides',
    order: 4,
  },
}

export const defaultCategory: CategoryConfig = {
  icon: 'memory:folder-open',
  label: 'Uncategorized',
}
