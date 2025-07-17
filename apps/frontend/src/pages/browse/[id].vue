<template>
  <div v-if="extension" class="space-y-12">
    <div
      class="overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
    >
      <div class="flex divide-x divide-neutral-700">
        <NuxtImg
          :src="`https://s3.blueprint.zip/extensions/${extension.identifier}.jpeg`"
          class="h-125 w-full object-cover"
        />
        <div
          class="min-w-100 flex flex-col justify-between divide-y divide-neutral-700"
        >
          <div class="p-4">
            <h1 class="truncate">
              {{ extension.name }}
            </h1>
            <p class="text-default-font/75 text-lg">
              {{ extension.summary }}
            </p>
          </div>
          <div class="bg-stripes h-full w-full" />
          <div class="divide-y divide-neutral-700">
            <a
              v-for="platform in availablePlatforms"
              :key="platform.key"
              :href="platform.data.url"
              target="_blank"
              rel="noopener noreferrer"
              class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
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
          </div>
        </div>
      </div>
    </div>

    <div
      class="flex gap-4 rounded-3xl border border-neutral-700 bg-neutral-950"
    >
      <div class="w-full p-4">e</div>
      <div class="min-w-100">
        <div class="p-4 ps-0">
          <div
            class="divide-y divide-neutral-700 overflow-hidden rounded-2xl border border-neutral-700"
          >
            <div class="flex items-center justify-between gap-2 p-2 text-lg">
              <span>Version history</span>
              <Icon name="pixelarticons:git-pull-request" />
            </div>
            <div class="max-h-75 overflow-y-scroll">
              <div
                v-for="version in extension.versions"
                class="flex items-center"
              >
                <div class="text-brand-50 min-w-8">
                  <SvgCommit
                    :type="
                      extension.versions.indexOf(version) == 0
                        ? 'start'
                        : 'default'
                    "
                    class="h-full w-full"
                  />
                </div>
                <div class="w-full py-2 pe-3">
                  <div class="flex items-center justify-between">
                    <p class="text-lg font-bold">{{ version.name }}</p>
                    <div
                      class="text-md text-default-font/50 flex items-center gap-1.5"
                    >
                      <Icon name="pixelarticons:arrow-bar-down" />
                      <p>{{ version.downloads }}</p>
                    </div>
                  </div>
                  <p class="">
                    {{ formatDate(version.created) }}
                  </p>
                </div>
              </div>
            </div>
            <div class="h-0 border-t-0">
              <div
                class="bg-linear-to-b z-5 -top-15 h-15 relative from-transparent to-neutral-950"
              ></div>
            </div>
          </div>
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

const formatPrice = (price: number, currency: string): string => {
  if (price === 0) {
    return 'Free'
  }

  return `${price.toFixed(2)} ${currency}`
}

const formatDate = (date: string): string => {
  const parsed = new Date(Date.parse(date))
  return new Intl.DateTimeFormat('en-GB', {
    dateStyle: 'full',
  }).format(parsed)
}

useSeoMeta({
  title: () => extension.value?.name,
  description: () => extension.value?.summary,
})
</script>
