<template>
  <div v-if="extension" class="space-y-12">
    <div
      class="overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
    >
      <div class="flex divide-x divide-neutral-700">
        <NuxtImg
          :src="`https://s3.blueprint.zip/extensions/${extension.identifier}.jpeg`"
          class="max-h-125 hidden aspect-video w-full object-cover lg:block"
        />
        <div
          class="lg:min-w-100 lg:max-w-100 flex w-full flex-col justify-between divide-y divide-neutral-700"
        >
          <div class="p-4">
            <h1 class="truncate !text-2xl lg:!text-3xl">
              {{ extension.name }}
            </h1>
            <p class="text-default-font/75 text-md lg:text-lg">
              {{ extension.summary }}
            </p>
          </div>
          <div class="h-full w-full">
            <div class="bg-stripes hidden h-full w-full lg:block" />
            <NuxtImg
              :src="`https://s3.blueprint.zip/extensions/${extension.identifier}.jpeg`"
              class="max-h-125 block aspect-video w-full object-cover lg:hidden"
            />
          </div>
          <div class="divide-y divide-neutral-700">
            <a
              v-for="platform in availablePlatforms"
              :key="platform.key"
              :href="
                platform.data.url +
                (platform.key == 'BUILTBYBIT' ? '?ref=581299&' : '?') +
                `utm_source=blueprint.zip&utm_medium=listing&utm_content=blueprint_${extension.identifier}`
              "
              target="_blank"
              rel="noopener noreferrer"
              class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
            >
              <div class="flex items-center gap-2 truncate">
                <component :is="platform.icon" />
                <div class="truncate text-xl font-semibold">
                  <p class="truncate">{{ platform.name }}</p>
                </div>
              </div>
              <p class="text-default-font/60">
                {{ formatPrice(platform.data.price, platform.data.currency) }}
              </p>
            </a>
          </div>
        </div>
      </div>
    </div>

    <div
      class="flex flex-col divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 lg:flex-row lg:divide-x"
    >
      <div class="w-full border-b border-neutral-700 lg:border-b-0">
        <template v-if="description?.body">
          <MDCRenderer
            class="prose-content p-4"
            :body="description?.body"
            :data="description?.data"
            :components="{
              img: 'ProseDisabled',
              script: 'ProseDisabled',
              style: 'ProseDisabled',
              iframe: 'ProseDisabled',
              object: 'ProseDisabled',
              embed: 'ProseDisabled',
            }"
          />
        </template>
        <template v-else>
          <div
            class="flex h-full w-full items-center justify-center align-middle"
          >
            <div class="max-w-100 p-4 text-center">
              <Icon name="pixelarticons:downasaur" :size="48" />
              <h1 class="mb-2.5">Missing description</h1>
              <p>
                {{ extension.author.name }} still has to write one. Until then,
                you'll have to deal with this dinosaur.
              </p>
            </div>
          </div>
        </template>
      </div>
      <div class="lg:min-w-100 lg:max-w-100">
        <div class="space-y-4 p-4">
          <div
            class="divide-y divide-neutral-700 overflow-hidden rounded-2xl border border-neutral-700"
          >
            <div class="flex items-center justify-between gap-2 p-2 text-lg">
              <span>Info</span>
              <Icon name="pixelarticons:align-left" />
            </div>
            <div class="p-2">
              <div class="flex items-center gap-1 font-bold">
                <Icon name="pixelarticons:calendar-month" />
                <div>
                  Released
                  <span class="text-default-font/60 font-normal">
                    <NuxtTime :datetime="extension.created" />
                  </span>
                </div>
              </div>
              <div class="flex items-center gap-1 font-bold">
                <Icon name="pixelarticons:user" />
                <div>
                  Created by
                  <span class="text-default-font/60 font-normal">
                    {{ extension.author.name }}
                  </span>
                </div>
              </div>
              <div class="flex items-center gap-1 font-bold">
                <Icon name="pixelarticons:chart-bar" />
                <div>
                  Used by
                  <span class="text-default-font/60 font-normal">
                    ~{{ extension.stats.panels }} installations
                  </span>
                </div>
              </div>

              <div class="flex gap-1 pt-2">
                <div
                  class="flex items-center gap-1 rounded-full border border-neutral-700 bg-neutral-900 px-2 py-0.5 text-xs"
                >
                  <template v-if="extension.type == 'extension'">
                    <Icon name="memory:cube" />
                    Extension
                  </template>
                  <template v-else-if="extension.type == 'theme'">
                    <Icon name="memory:image" />
                    Theme
                  </template>
                </div>
                <div
                  class="flex items-center gap-1 rounded-full border border-neutral-700 bg-neutral-900 px-2 py-0.5 text-xs"
                >
                  <Icon name="pixelarticons:label" />
                  {{ extension.identifier }}
                </div>
              </div>
            </div>
          </div>
          <div
            class="divide-y divide-neutral-700 overflow-hidden rounded-2xl border border-neutral-700"
          >
            <div class="flex items-center justify-between gap-2 p-2 text-lg">
              <span>Version history</span>
              <Icon name="pixelarticons:git-pull-request" />
            </div>
            <div class="max-h-75 overflow-y-scroll">
              <div
                v-if="extension.versions[0]"
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
                      class="text-md text-default-font/60 flex items-center gap-1.5"
                    >
                      <Icon name="pixelarticons:download" />
                      <p>{{ version.downloads }}</p>
                    </div>
                  </div>
                  <p class="text-default-font/60">
                    <NuxtTime :datetime="version.created" relative />
                  </p>
                </div>
              </div>
              <div class="p-2" v-else>
                <p class="text-default-font/60">
                  Our robots are hard at work collecting information about this
                  extension, come back later!
                </p>
              </div>
            </div>
            <div class="h-0 border-t-0" v-if="extension.versions[1]">
              <div
                class="bg-linear-to-b z-5 -top-15 h-15 relative from-transparent to-neutral-950"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { parseMarkdown } from '@nuxtjs/mdc/runtime'
import type { MDCParserResult } from '@nuxtjs/mdc'

const { sanitizeAst } = useMdcSanitizer()
const route = useRoute()

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

const { data: extension } = await useAsyncData<Extension | null>(
  `extension-${route.params.id}`,
  () => $fetch(`/api/extensions/${route.params.id}`),
  {
    server: false,
    default: () => null,
  }
)

useSeoMeta({
  title: () => extension.value?.name,
  description: () => extension.value?.summary,
})

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

const { data: description } = await useAsyncData(
  `description-${route.params.id}`,
  async (): Promise<MDCParserResult | null> => {
    if (!extension.value?.description) return null

    try {
      const parsed = await parseMarkdown(extension.value.description)
      return {
        ...parsed,
        body: sanitizeAst(parsed.body),
      }
    } catch (error) {
      console.warn('Failed to parse markdown:', error)
      return null
    }
  },
  {
    watch: [() => extension.value?.description],
    immediate: !!extension.value?.description,
  }
)
</script>
