<template>
  <div v-if="extension">
    <div class="space-y-4">
      <h1
        class="!display leading-14 md:leading-18 truncate !text-5xl !font-normal md:!text-6xl"
      >
        {{ extension.name }}
      </h1>
      <p class="text-default-font/60 text-lg">
        {{ extension.summary }}
      </p>
    </div>
    <div
      class="divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700"
    >
      <div class="p-4">
        <NuxtImg
          :src="`https://s3.blueprint.zip/extensions/${extension.identifier}.jpeg`"
          class="h-125 w-full rounded-2xl border border-neutral-700 object-cover"
        />
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
