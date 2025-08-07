export interface CategoryConfig {
  icon: string
  label?: string
  order?: number
}

export const docsCategories: Record<string, CategoryConfig> = {
  concepts: {
    icon: 'memory:lightbulb',
    label: 'Concepts',
    order: 1,
  },
  cli: {
    icon: 'memory:terminal',
    label: 'CLI',
    order: 2,
  },
  configs: {
    icon: 'memory:script',
    label: 'Configurations',
    order: 3,
  },
  lib: {
    icon: 'memory:application-code',
    label: 'Extension Library',
    order: 4,
  },
}

export const defaultCategory: CategoryConfig = {
  icon: 'memory:folder-open',
  label: 'Uncategorized',
}
