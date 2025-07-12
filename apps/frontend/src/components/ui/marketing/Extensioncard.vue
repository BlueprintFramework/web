<template>
  <div :class="props.class">
    <div class="space-y-4 p-4">
      <NuxtImg
        v-if="props.extension"
        :src="`https://s3.blueprint.zip/extensions/lowres/${props.extension.identifier}.jpeg`"
        class="aspect-video w-full rounded-2xl border border-neutral-700"
      />
      <div
        v-else
        class="aspect-video w-full overflow-hidden rounded-2xl border border-neutral-700 bg-neutral-950"
      >
        <div class="h-full w-full animate-pulse bg-neutral-900" />
      </div>

      <div v-if="props.extension" class="space-y-1.5">
        <h2 class="truncate">{{ props.extension.name }}</h2>
        <p class="text-default-font/60 truncate">
          {{ props.extension.summary }}
        </p>
      </div>
      <div v-else class="space-y-1.5">
        <div class="w-18 max-w-2/4 h-6 rounded-2xl bg-neutral-900" />
        <div class="max-w-2/3 h-6 w-40 rounded-2xl bg-neutral-900" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  extension?: Extension
  class?: string
}>()
</script>
