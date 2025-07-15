<template>
  <div class="flex items-center gap-3">
    <Icon
      :name="categoryConfig.icon"
      :size="48"
      mode="svg"
      class="block !text-4xl md:!text-5xl"
    />
    <h1 class="!text-3xl md:!text-4xl">{{ categoryConfig.label }}</h1>
  </div>

  <div
    class="flex flex-col gap-4 rounded-3xl border border-neutral-700 bg-neutral-950 p-4"
  >
    <NuxtLink v-for="guide in categoryData" :key="guide.id" :to="guide.path">
      <div
        class="group grid grid-cols-1 divide-neutral-700 overflow-hidden rounded-2xl border border-neutral-700 transition-colors hover:bg-neutral-900 md:grid-cols-2 md:divide-x"
      >
        <div class="flex h-full p-4">
          <div class="space-y-2 self-center overflow-hidden text-nowrap">
            <div class="flex items-center gap-2">
              <span
                class="group-hover:text-brand-50 h2 lg:h1 overflow-hidden text-ellipsis transition-colors"
              >
                {{ guide.title }}
              </span>
              <Icon
                name="memory:chevron-right"
                :size="32"
                class="text-brand-50 opacity-0 transition-opacity group-hover:opacity-100"
              />
            </div>
            <p class="text-default-font/60 overflow-hidden text-ellipsis">
              {{ guide?.description || 'No description provided' }}
            </p>
          </div>
        </div>
        <div
          class="order-first overflow-hidden border-b border-neutral-700 md:order-last md:border-b-0"
        >
          <NuxtImg
            :src="`/img/guides/thumbnails/${guide.thumbnail || 'default.jpeg'}`"
            :height="300"
            :width="1280"
            class="group-hover:scale-102 h-full w-full object-cover transition-transform"
          />
        </div>
      </div>
    </NuxtLink>
  </div>
</template>

<script setup>
import { guidesCategories } from '~/assets/guides.config'

const route = useRoute()
const categoryKey = route.params.category
const categoryConfig = guidesCategories[categoryKey]

const { data: categoryData } = await useAsyncData(
  `guides-list-${categoryKey}`,
  () => queryCollection('guides').where('category', '=', categoryKey).all()
)

if (!categoryData.value || !categoryConfig) {
  throw createError({
    statusCode: 404,
    statusMessage: 'Page not found',
  })
}
</script>
