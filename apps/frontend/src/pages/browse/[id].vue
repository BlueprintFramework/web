<template>
  <div v-if="extension">
    <div
      class="overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
    >
      <div class="flex divide-x divide-neutral-700">
        <div class="w-full">
          <NuxtImg
            :src="`https://s3.blueprint.zip/extensions/${extension.identifier}.jpeg`"
            class="h-100 w-full border-b border-neutral-700 object-cover"
          />
          <div class="w-full space-y-3 p-8">
            <h1 class="truncate">
              {{ extension.name }}
            </h1>
            <p class="text-default-font/75 text-lg">
              {{ extension.summary }}
            </p>
            <div class="text-default-font/60 flex gap-3">
              <p class="flex items-center gap-2">
                <Icon name="memory:account" />
                {{ extension.author.name }}
              </p>
              <span>•</span>
              <p class="flex items-center gap-2">
                <Icon name="memory:chart-bar" />
                {{ extension.stats.panels }}
              </p>
            </div>
          </div>
        </div>
        <div class="min-w-100">
          <a
            v-for="platform in availablePlatforms"
            :key="platform.key"
            :href="platform.data.url"
            target="_blank"
            rel="noopener noreferrer"
            class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between border-b border-neutral-700 bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
          >
            <div class="flex items-center gap-2">
              <component :is="platform.icon" />
              <div class="text-xl font-semibold">
                <span>{{ platform.name }}</span>
              </div>
            </div>
            <span class="text-default-font/60">
              {{ formatPrice(platform.data.price, platform.data.currency) }}
            </span>
          </a>

          <div class="bg-stripes h-full w-full" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()

const { data: extension, pending } = await useAsyncData<Extension>(
  `extension-${route.params.id}`,
  () => $fetch<Extension>(`/api/extensions/${route.params.id}`),
  {
    server: false,
  }
)

// Platform configuration with imported components
const platformConfig = {
  BUILTBYBIT: {
    name: 'BuiltByBit',
    icon: resolveComponent('SvgBuiltbybit'),
  },
  SOURCEXCHANGE: {
    name: 'sourceXchange',
    icon: resolveComponent('SvgSourcexchange'),
  },
  GITHUB: {
    name: 'GitHub',
    icon: resolveComponent('SvgGithub'),
  },
} as const

// Computed property to get available platforms with their config
const availablePlatforms = computed(() => {
  if (!extension.value?.platforms) {
    return []
  }

  return Object.entries(extension.value.platforms)
    .filter(([_, platformData]) => platformData)
    .map(([platformKey, platformData]) => ({
      key: platformKey,
      name:
        platformConfig[platformKey as keyof typeof platformConfig]?.name ||
        platformKey,
      icon:
        platformConfig[platformKey as keyof typeof platformConfig]?.icon ||
        resolveComponent('SvgGithub'),
      data: platformData,
    }))
})

// Helper function to format price with currency as text
const formatPrice = (price: number, currency: string): string => {
  // Show "Free" for zero price, otherwise format with currency as text
  if (price === 0) {
    return 'Free'
  }

  return `${price.toFixed(2)} ${currency}`
}

useSeoMeta({
  title: () => extension.value?.name,
  description: () => extension.value?.summary,
})
</script>
