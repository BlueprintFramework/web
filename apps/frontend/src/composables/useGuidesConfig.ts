import {
  guidesCategories,
  defaultCategory,
  type CategoryConfig,
} from '~/assets/guides.config'

export const useGuidesConfig = () => {
  const getCategoryConfig = (category: string): CategoryConfig => {
    const categoryKey = category?.toLowerCase() || 'uncategorized'
    return guidesCategories[categoryKey] || defaultCategory
  }

  const getCategoryLabel = (category: string): string => {
    const config = getCategoryConfig(category)
    return config.label || category || 'Uncategorized'
  }

  const getCategoryIcon = (category: string): string => {
    return getCategoryConfig(category).icon
  }

  return {
    categories: guidesCategories,
    defaultCategory,
    getCategoryConfig,
    getCategoryLabel,
    getCategoryIcon,
  }
}
