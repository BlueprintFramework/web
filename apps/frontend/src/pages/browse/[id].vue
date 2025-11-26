<template>
  <div v-if="extension" class="space-y-12">
    <div
      class="parent-focus overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 transition-colors focus-within:border-neutral-500"
    >
      <div
        class="apply-parent-focus flex divide-x divide-neutral-700 transition-colors"
      >
        <NuxtImg
          :src="extension.banner.fullres"
          class="max-h-125 hidden aspect-video w-full object-cover transition-colors lg:block"
        />
        <div
          class="lg:min-w-100 lg:max-w-100 apply-parent-focus flex w-full flex-col justify-between divide-y divide-neutral-700 transition-colors"
        >
          <div class="p-4 transition-colors">
            <div class="flex justify-between gap-2">
              <h1 class="truncate !text-2xl lg:!text-3xl">
                {{ extension.name }}
              </h1>
              <NuxtLink
                :to="`/app/extensions/${extension.id}`"
                v-if="user?.admin || user?.id == extension.author.id"
                tabindex="-1"
              >
                <ElementsButtonSmall> Edit </ElementsButtonSmall>
              </NuxtLink>
            </div>
            <p class="text-default-font/75 text-md lg:text-lg">
              {{ extension.summary }}
            </p>
          </div>
          <div class="h-full w-full transition-colors">
            <div class="bg-stripes hidden h-full w-full lg:block" />
            <NuxtImg
              :src="extension.banner.fullres"
              class="max-h-125 block aspect-video w-full object-cover lg:hidden"
            />
          </div>
          <div
            class="apply-parent-focus divide-y divide-neutral-700 transition-colors"
          >
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
              class="text-default-font hover:text-brand-50 focus:text-brand-50 flex w-full cursor-pointer items-center justify-between bg-neutral-950 px-4 py-3 outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
              @mousedown.prevent
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
              card: 'Card',
              h1: 'ProseH2',
              h2: 'ProseH3',
              h3: 'ProseH4',
              h4: 'ProseH5',
              h5: 'ProseH6',
              h6: 'ProseH6',
              img: 'Externalimg',
              script: 'ProseDisabled',
              style: 'ProseDisabled',
              iframe: 'ProseDisabled',
              object: 'ProseDisabled',
              embed: 'ProseDisabled',
              pre: 'ProsePreSafe',
            }"
          />
        </template>
        <template v-else>
          <div
            class="flex h-full w-full items-center justify-center align-middle"
          >
            <div class="md:max-w-100 p-4 text-left md:text-center">
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
                    ~{{ extension.stats.panels }} Blueprint installations
                  </span>
                </div>
              </div>
              <div class="flex items-center gap-1 font-bold">
                <Icon name="pixelarticons:calendar-month" />
                <div>
                  Released
                  <span class="text-default-font/60 font-normal">
                    <NuxtTime :datetime="extension.created" />
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
              <span>Moodmeter™</span>
              <Icon name="pixelarticons:human" />
            </div>
            <div v-if="moodmeter == 'unknown'" class="p-2">
              <span class="text-default-font/60">
                This extension does not have enough reviews for a Moodmeter
                rating.
              </span>
            </div>
            <div v-if="moodmeter != 'unknown'" class="p-2">
              <div class="flex flex-col items-center py-3">
                <div class="flex items-center gap-2">
                  <div>
                    <Icon
                      :name="moodmeterIcon"
                      :size="52"
                      mode="svg"
                      class="size-8 md:size-[52px]"
                    />
                  </div>
                  <div>
                    <p v-if="moodmeter == 'love'">
                      Users
                      <b class="font-extrabold text-purple-400">love</b>
                      this extension!
                    </p>
                    <p v-if="moodmeter == 'really like'">
                      Users
                      <b class="font-extrabold text-green-400">really like</b>
                      this extension
                    </p>
                    <p v-if="moodmeter == 'like'">
                      Users
                      <b class="text-brand-50 font-extrabold">like</b>
                      this extension
                    </p>
                    <p v-if="moodmeter == 'meh'">
                      Users feel
                      <b class="font-extrabold text-yellow-200">meh</b>
                      about this extension
                    </p>
                    <p v-if="moodmeter == 'dislike'">
                      Users
                      <b class="font-extrabold text-red-400">dislike</b>
                      this extension
                    </p>
                    <p class="text-default-font/60">
                      {{ totalReviews }} reviews submitted
                    </p>
                  </div>
                </div>
              </div>
            </div>
            <div v-if="moodmeter != 'unknown'" class="p-2">
              <table class="w-full">
                <thead>
                  <tr>
                    <th>
                      <Icon
                        name="pixelarticons:mood-sad"
                        :class="
                          moodmeter == 'dislike'
                            ? 'text-red-400'
                            : 'text-default-font/60'
                        "
                      />
                    </th>
                    <th>
                      <Icon
                        name="pixelarticons:mood-neutral"
                        :class="
                          moodmeter == 'meh'
                            ? 'text-yellow-200'
                            : 'text-default-font/60'
                        "
                      />
                    </th>
                    <th>
                      <Icon
                        name="pixelarticons:mood-happy"
                        :class="
                          moodmeter == 'like'
                            ? 'text-brand-50'
                            : 'text-default-font/60'
                        "
                      />
                    </th>
                    <th>
                      <Icon
                        name="pixelarticons:human-handsup"
                        :class="
                          moodmeter == 'really like'
                            ? 'text-green-400'
                            : 'text-default-font/60'
                        "
                      />
                    </th>
                    <th>
                      <Icon
                        name="pixelarticons:heart"
                        :class="
                          moodmeter == 'love'
                            ? 'text-purple-400'
                            : 'text-default-font/60'
                        "
                      />
                    </th>
                  </tr>
                </thead>
                <tbody>
                  <tr class="divide-x divide-neutral-700">
                    <td>
                      <div
                        class="h-4 w-full rounded-s-full border-y border-s border-neutral-700"
                        :class="
                          moodmeter == 'dislike'
                            ? 'bg-red-400'
                            : 'bg-neutral-900'
                        "
                      />
                    </td>
                    <td>
                      <div
                        class="h-4 w-full border-y border-neutral-700"
                        :class="
                          moodmeter == 'meh'
                            ? 'bg-yellow-200'
                            : 'bg-neutral-900'
                        "
                      />
                    </td>
                    <td>
                      <div
                        class="h-4 w-full border-y border-neutral-700"
                        :class="
                          moodmeter == 'like' ? 'bg-brand-50' : 'bg-neutral-900'
                        "
                      />
                    </td>
                    <td>
                      <div
                        class="h-4 w-full border-y border-neutral-700"
                        :class="
                          moodmeter == 'really like'
                            ? 'bg-green-400'
                            : 'bg-neutral-900'
                        "
                      />
                    </td>
                    <td>
                      <div
                        class="h-4 w-full rounded-e-full border-y border-e border-neutral-700"
                        :class="
                          moodmeter == 'love'
                            ? 'bg-purple-400'
                            : 'bg-neutral-900'
                        "
                      />
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <div
            class="divide-y divide-neutral-700 overflow-hidden rounded-2xl border border-neutral-700 transition-colors focus-within:divide-neutral-500 focus-within:border-neutral-500"
          >
            <div
              class="flex items-center justify-between gap-2 p-2 text-lg transition-colors"
            >
              <span>Version history</span>
              <Icon name="pixelarticons:git-pull-request" />
            </div>
            <div
              class="max-h-75 overflow-y-scroll bg-neutral-950 outline-0 transition-colors focus:bg-neutral-900"
            >
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

const route = useRoute()
const { user } = useAuth()

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

const totalReviews = computed<unknown | number>(() =>
  Object.values(extension.value?.platforms || {}).reduce(
    (sum, platform) => sum + (platform.reviews || 0),
    0
  )
)
const platformsWithRatings = computed(() =>
  Object.values(extension.value?.platforms || {}).filter((x) => x.rating)
)
const averageRating = computed<unknown | number>(() =>
  platformsWithRatings.value.length > 0
    ? platformsWithRatings.value.reduce(
        (sum, platform) => sum + (platform.rating || 0),
        0
      ) / platformsWithRatings.value.length
    : 0
)

const moodmeter = computed(() => {
  if ((totalReviews.value as number) < 3) return 'unknown'
  if ((extension.value?.stats.panels || 0) < 20) return 'unknown'

  const rating = averageRating.value as number
  const adjustment = Math.log10((totalReviews.value as number) / 5) * 0.12

  if (!moodmeterLimited.value) {
    switch (true) {
      case rating < 2.5 + adjustment:
        return 'dislike'
      case rating >= 2.5 + adjustment && rating < 3.5 + adjustment:
        return 'meh'
      case rating >= 3.5 + adjustment && rating < 4.3 + adjustment:
        return 'like'
      case rating >= 4.3 + adjustment && rating < 4.7 + adjustment:
        return 'really like'
      case rating >= 4.7 + adjustment:
        return 'love'
    }
  } else {
    switch (true) {
      case rating < 3.5 + adjustment:
        return 'meh'
      case rating >= 3.5 + adjustment && rating < 4.0 + adjustment:
        return 'like'
      case rating >= 4.0 + adjustment:
        return 'really like'
    }
  }
})
const moodmeterLimited = computed(() => (totalReviews.value as number) < 12)
const moodmeterIcon = computed(() => {
  switch (moodmeter.value) {
    default:
      return 'pixelarticons:cellular-signal-off'
    case 'dislike':
      return 'pixelarticons:mood-sad'
    case 'meh':
      return 'pixelarticons:mood-neutral'
    case 'like':
      return 'pixelarticons:mood-happy'
    case 'really like':
      return 'pixelarticons:human-handsup'
    case 'love':
      return 'pixelarticons:heart'
  }
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
      const parsed = await parseMarkdown(extension.value.description, {
        rehype: { options: { allowDangerousHtml: false } },
      })
      return {
        ...parsed,
        body: parsed.body,
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

<style scoped>
@reference "~/assets/css/main.css";

.parent-focus:focus-within .apply-parent-focus {
  @apply divide-neutral-500 border-neutral-500;
}
</style>
