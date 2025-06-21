export interface CategoryConfig {
  icon: string
  label?: string
  order?: number
}

export const docsCategories: Record<string, CategoryConfig> = {
  cli: {
    icon: 'memory:terminal',
    label: 'CLI',
    order: 1,
  },

  configs: {
    icon: 'memory:script',
    label: 'Configurations',
    order: 2,
  },

  lib: {
    icon: 'memory:application-code',
    label: 'Extension Library',
    order: 3,
  },
}

export const defaultCategory: CategoryConfig = {
  icon: 'memory:folder-open',
  label: 'Uncategorized',
}
