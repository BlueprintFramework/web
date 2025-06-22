<template>
  <div class="flex">
    <div class="flex">
      <div class="w-76">
        <div
          class="w-76 fixed z-10 h-full flex-col space-y-4 overflow-y-scroll bg-neutral-950"
        >
          <div
            class="bg-linear-to-b sticky top-0 space-y-4 border-b border-neutral-700 from-neutral-950/50 to-transparent p-4 backdrop-blur-sm"
          >
            <div class="flex items-center justify-between">
              <BrandWordmark />
              <NuxtLink
                to="/docs"
                class="hover:text-brand-50 text-default-font/50 text-lg transition-colors"
                :class="{ '!text-default-font': route.path == '/docs' }"
              >
                Docs
              </NuxtLink>
            </div>
            <UiFormInput
              v-model="form.search"
              name="search"
              type="text"
              :rules="[]"
              leading-icon="memory:search"
              placeholder="Search.."
              @validate="void"
            />
          </div>
          <div class="space-y-4 px-4 pb-4">
            <div
              v-for="(categoryData, category) in filteredCategories"
              :key="category"
              class="divide-y divide-neutral-700 rounded-2xl border border-neutral-700"
            >
              <div class="flex items-center gap-1.5 p-2 font-bold">
                <Icon
                  :name="categoryData.icon"
                  :size="22"
                  mode="svg"
                  class="block"
                />
                <span>{{ category }}</span>
              </div>
              <div class="space-y-2 p-2">
                <NuxtLink
                  v-for="doc in categoryData.docs"
                  :key="doc.id"
                  :to="doc.path"
                  class="hover:text-brand-50 text-default-font/60 block w-full text-start transition-colors"
                  :class="{
                    '!text-default-font': route.path == doc.path,
                  }"
                >
                  <span>{{ doc.title }}</span>
                </NuxtLink>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div>
        <div
          class="bg-linear-to-b fixed h-full w-[1px] from-neutral-800 via-neutral-500 to-neutral-800"
        />
      </div>
    </div>
    <div class="max-w-200 container space-y-12 overflow-hidden px-4 py-12">
      <slot />
    </div>
  </div>

  <div
    class="fixed inset-0 top-0 -z-10 h-[50vh] w-full bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:30px_30px] bg-[position:-5px_-5px]"
  >
    <div class="bg-linear-to-b h-full w-full from-transparent to-neutral-950" />
  </div>
</template>

<script setup lang="ts">
import { docsCategories, defaultCategory } from '~/assets/docs.config'

const route = useRoute()
const form = ref({
  search: '',
})

const { data: docs } = await useAsyncData('docs-sidebar', () => {
  return queryCollection('docs').all()
})

// Group docs by category
const groupedDocs = computed(() => {
  if (!docs.value) return {}

  const grouped = docs.value.reduce(
    (acc, doc) => {
      const categoryKey = doc.category?.toLowerCase() || 'general'
      const categoryConfig = docsCategories[categoryKey] || defaultCategory
      const categoryLabel = categoryConfig.label || doc.category || 'General'

      if (!acc[categoryLabel]) {
        acc[categoryLabel] = {
          icon: categoryConfig.icon,
          order: categoryConfig.order || 999,
          docs: [],
        }
      }
      acc[categoryLabel].docs.push(doc)
      return acc
    },
    {} as Record<string, { icon: string; order: number; docs: any[] }>
  )

  Object.values(grouped).forEach((category) => {
    category.docs.sort((a, b) => {
      if (a.order !== undefined && b.order !== undefined) {
        return a.order - b.order
      }
      if (a.order !== undefined) return -1
      if (b.order !== undefined) return 1
      return a.title.localeCompare(b.title)
    })
  })

  const sortedGrouped = Object.entries(grouped)
    .sort(([, a], [, b]) => a.order - b.order)
    .reduce(
      (acc, [key, value]) => {
        acc[key] = value
        return acc
      },
      {} as typeof grouped
    )

  return sortedGrouped
})

// Filter categories based on search
const filteredCategories = computed(() => {
  const search = form.value.search.toLowerCase()
  if (!search) return groupedDocs.value

  const filtered: Record<string, { icon: string; order: number; docs: any[] }> =
    {}

  Object.entries(groupedDocs.value).forEach(([category, data]) => {
    const matchingDocs = data.docs.filter(
      (doc) =>
        doc.title.toLowerCase().includes(search) ||
        doc.description?.toLowerCase().includes(search) ||
        category.toLowerCase().includes(search)
    )

    if (matchingDocs.length > 0) {
      filtered[category] = {
        icon: data.icon,
        order: data.order,
        docs: matchingDocs,
      }
    }
  })

  return filtered
})
</script>
