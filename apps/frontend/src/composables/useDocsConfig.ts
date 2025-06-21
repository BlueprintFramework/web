import {
  docsCategories,
  defaultCategory,
  type CategoryConfig,
} from '~/assets/docs.config'

export const useDocsConfig = () => {
  const getCategoryConfig = (category: string): CategoryConfig => {
    const categoryKey = category?.toLowerCase() || 'uncategorized'
    return docsCategories[categoryKey] || defaultCategory
  }

  const getCategoryLabel = (category: string): string => {
    const config = getCategoryConfig(category)
    return config.label || category || 'Uncategorized'
  }

  const getCategoryIcon = (category: string): string => {
    return getCategoryConfig(category).icon
  }

  return {
    categories: docsCategories,
    defaultCategory,
    getCategoryConfig,
    getCategoryLabel,
    getCategoryIcon,
  }
}
