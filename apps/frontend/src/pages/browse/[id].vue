<template>
  <div v-if="extension">
    <div class="flex gap-4">
      <div class="w-full">
        <div
          class="w-full overflow-hidden rounded-3xl border border-neutral-700"
        >
          <div class="p-4">
            <NuxtImg
              :src="`https://s3.blueprint.zip/extensions/${extension.identifier}.jpeg`"
              class="aspect-video w-full rounded-2xl border border-neutral-700 object-cover"
            />
          </div>
        </div>
      </div>

      <div class="min-w-100">
        <div
          class="w-full divide-y divide-neutral-700 rounded-3xl border border-neutral-700"
        >
          <div class="space-y-2 p-4">
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
          <div class="p-4">wrrf</div>
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
