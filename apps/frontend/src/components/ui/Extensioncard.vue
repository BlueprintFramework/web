<template>
  <div :class="props.class">
    <NuxtLink
      :to="props.extension ? `/browse/${props.extension.identifier}` : ''"
    >
      <div
        class="group space-y-4 bg-neutral-950 p-4 transition-colors hover:bg-neutral-900"
      >
        <div
          v-if="props.extension"
          class="aspect-video w-full overflow-hidden rounded-2xl border border-neutral-700"
        >
          <NuxtImg
            :src="`https://s3.blueprint.zip/extensions/lowres/${props.extension.identifier}.jpeg`"
            class="h-full w-full transition-transform group-hover:scale-105"
          />
        </div>
        <div
          v-else
          class="aspect-video w-full overflow-hidden rounded-2xl border border-neutral-700 bg-neutral-950"
        >
          <div class="h-full w-full animate-pulse bg-neutral-900" />
        </div>

        <div v-if="props.extension" class="space-y-1.5">
          <div class="flex flex-row items-center justify-between gap-2">
            <h2 class="group-hover:text-brand-50 truncate transition-colors">
              {{ props.extension.name }}
            </h2>
            <div class="flex gap-2">
              <div
                class="text-nowrap rounded-full border border-neutral-700 bg-neutral-900 px-2 py-0.5 text-xs"
                v-if="lowestPrice?.price"
              >
                {{ formattedCurrency + lowestPrice?.price.toFixed(2) }}
              </div>
              <div
                class="flex items-center gap-1 rounded-full border border-neutral-700 bg-neutral-900 px-2 py-0.5 text-xs"
              >
                <Icon name="memory:chart-bar" />
                {{ props.extension.stats.panels }}
              </div>
            </div>
          </div>
          <p class="text-default-font/60 truncate">
            {{ props.extension.summary }}
          </p>
        </div>
        <div v-else class="space-y-1.5">
          <div class="w-18 max-w-2/4 h-6 rounded-2xl bg-neutral-900" />
          <div class="max-w-2/3 h-6 w-40 rounded-2xl bg-neutral-900" />
        </div>
      </div>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  extension?: Extension
  class?: string
}>()

const lowestPrice = computed(() => {
  if (!props.extension?.platforms) {
    return null
  }

  const platformsArray = Object.values(
    props.extension.platforms
  ) as unknown as ExtensionPlatforms[]

  if (platformsArray.length === 0) {
    return null
  }

  const lowest: ExtensionPlatforms = platformsArray.reduce((lowest, current) =>
    current.price < lowest.price ? current : lowest
  )

  return {
    price: lowest.price,
    currency: lowest.currency,
  }
})

const formattedCurrency = {
  EUR: '€' as string,
  USD: '$' as string,
  GBP: '£' as string,
}[lowestPrice.value?.currency || 'EUR']
</script>
