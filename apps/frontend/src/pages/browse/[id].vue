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
          <div class="space-y-3 p-8">
            <h1 class="truncate">
              {{ extension.name }}
            </h1>
            <p class="text-default-font/60 text-lg">
              {{ extension.summary }}
            </p>
            <p class="text-default-font/50 flex items-center gap-2">
              <Icon name="memory:account" />
              {{ extension.author.name }}
            </p>
          </div>
        </div>
        <div class="min-w-100">
          <button
            type="submit"
            class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between border-b border-neutral-700 bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
          >
            <div class="text-xl font-semibold">
              <span>Buy on BuiltByBit</span>
            </div>
            <Icon name="memory:chevron-right" mode="svg" :size="24" />
          </button>
          <button
            type="submit"
            class="text-default-font hover:text-brand-50 flex w-full cursor-pointer items-center justify-between border-b border-neutral-700 bg-neutral-950 px-4 py-3 transition-colors hover:bg-neutral-900"
          >
            <div class="text-xl font-semibold">
              <span>Buy on sourceXchange</span>
            </div>
            <Icon name="memory:chevron-right" mode="svg" :size="24" />
          </button>
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

useSeoMeta({
  title: () => extension.value?.name,
  description: () => extension.value?.summary,
})
</script>
