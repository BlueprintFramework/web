<template>
  <div
    class="flex divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 transition-colors focus-within:divide-neutral-500 focus-within:border-neutral-500 lg:divide-x"
  >
    <div
      class="flex-1/2 divide-y divide-neutral-700 transition-colors focus-within:divide-neutral-500"
    >
      <div class="space-y-3 p-4 transition-colors md:p-8">
        <h1>Guides</h1>
        <span class="opacity-75">
          Learn how to interact with our platform. From installing Blueprint to
          developing extensions, we're here to help you get started.
        </span>
      </div>
      <div class="p-4 transition-colors md:p-8">
        <ElementsFormInput
          v-model="form.search"
          name="search"
          type="text"
          :rules="[]"
          leading-icon="memory:search"
          placeholder="Search.."
        />
      </div>
    </div>
    <div class="flex-1/2 relative hidden transition-colors lg:block">
      <NuxtImg
        src="/img/guides.jpeg"
        :width="1280"
        :height="600"
        class="absolute inset-0 h-full w-full object-cover"
      />
    </div>
  </div>

  <div class="space-y-12">
    <div
      v-for="(categoryData, category) in filteredCategories"
      :key="category"
      class="guide-category divide-y divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 transition-colors focus-within:divide-neutral-500 focus-within:border-neutral-500"
    >
      <div class="flex justify-between transition-colors">
        <div class="hidden items-center gap-1.5 p-4 text-2xl font-bold sm:flex">
          <Icon :name="categoryData.icon" :size="32" mode="svg" class="block" />
          <span>{{ category }}</span>
        </div>
        <NuxtLink
          :to="`/guides/category/${categoryData.key}`"
          class="hover:text-brand-50 flex w-full items-center gap-1.5 p-4 text-2xl font-bold transition-colors hover:bg-neutral-900 sm:hidden"
          tabindex="-1"
          @mousedown.prevent
        >
          <Icon :name="categoryData.icon" :size="32" mode="svg" class="block" />
          <span>{{ category }}</span>
        </NuxtLink>
        <div
          class="category-link hidden border-s border-neutral-700 transition-colors sm:inline"
        >
          <NuxtLink
            :to="`/guides/category/${categoryData.key}`"
            @mousedown.prevent
            class="group outline-0"
          >
            <button
              class="text-default-font group-focus:text-brand-50 hover:text-brand-50 flex h-full cursor-pointer items-center justify-between gap-1 bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900 group-focus:bg-neutral-900"
              tabindex="-1"
            >
              <span class="hidden text-xl font-semibold md:inline">
                View category
              </span>
              <Icon name="memory:chevron-right" mode="svg" :size="24" />
            </button>
          </NuxtLink>
        </div>
      </div>
      <div
        class="flex gap-4 overflow-x-scroll p-4 outline-0 transition-colors focus:bg-neutral-900"
      >
        <NuxtLink
          v-for="guide in categoryData.guides"
          :key="guide.id"
          :to="guide.path"
          class="min-w-95 group w-1/4 outline-0"
          @mousedown.prevent
        >
          <div
            class="overflow-hidden rounded-2xl border border-neutral-700 bg-neutral-950 transition-colors hover:bg-neutral-900 group-focus:border-neutral-500 group-focus:bg-neutral-900"
          >
            <NuxtImg
              :src="`/img/guides/thumbnails/${guide.thumbnail || 'default.jpeg'}`"
              :height="100"
              :width="450"
              class="border-b border-neutral-700 object-cover"
            />
            <div class="space-y-2 overflow-hidden text-nowrap p-4">
              <h2
                class="group-hover:text-brand-50 group-focus:text-brand-50 overflow-hidden text-ellipsis transition-colors"
              >
                {{ guide.title }}
              </h2>
              <p class="text-default-font/60 overflow-hidden text-ellipsis">
                {{ guide?.description || 'No description provided' }}
              </p>
            </div>
          </div>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { guidesCategories, defaultCategory } from '~/assets/guides.config'

const form = ref({
  search: '',
})

const { data: guides } = await useAsyncData('guides-list', () => {
  return queryCollection('guides').all()
})

// Group guides by category
const groupedGuides = computed(() => {
  if (!guides.value) return {}

  const grouped = guides.value.reduce(
    (acc, guide) => {
      const categoryKey = guide.category?.toLowerCase() || 'general'
      const categoryConfig = guidesCategories[categoryKey] || defaultCategory
      const categoryLabel = categoryConfig.label || guide.category || 'General'

      if (!acc[categoryLabel]) {
        acc[categoryLabel] = {
          icon: categoryConfig.icon,
          order: categoryConfig.order || 999,
          key: categoryKey,
          guides: [],
        }
      }
      if (!guide.unlisted) {
        acc[categoryLabel].guides.push(guide)
      }
      return acc
    },
    {} as Record<
      string,
      { icon: string; order: number; key: string; guides: any[] }
    >
  )

  Object.values(grouped).forEach((category) => {
    category.guides.sort((a, b) => {
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
  if (!search) return groupedGuides.value

  const filtered: Record<
    string,
    { icon: string; order: number; key: string; guides: any[] }
  > = {}

  Object.entries(groupedGuides.value).forEach(([category, data]) => {
    const matchingGuides = data.guides.filter(
      (guide) =>
        guide.title.toLowerCase().includes(search) ||
        guide.description?.toLowerCase().includes(search) ||
        category.toLowerCase().includes(search)
    )

    if (matchingGuides.length > 0) {
      filtered[category] = {
        icon: data.icon,
        order: data.order,
        key: data.key,
        guides: matchingGuides,
      }
    }
  })

  return filtered
})
</script>

<style scoped>
@reference "~/assets/css/main.css";

.guide-category:focus-within .category-link {
  @apply border-neutral-500;
}
</style>
