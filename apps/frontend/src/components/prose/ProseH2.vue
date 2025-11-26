<template>
  <h2 :id="props.id" style="font-size: 31px" class="mb-4 mt-12">
    <a
      v-if="props.id && generate"
      :href="`#${props.id}`"
      class="hover:text-brand-50 transition-colors"
    >
      <slot />
    </a>
    <slot v-else />
  </h2>
</template>

<script setup lang="ts">
import { computed, useRuntimeConfig } from '#imports'

const props = defineProps<{
  id?: string

  // Disable class and style props
  class?: string
  style?: string
}>()

const { headings } = useRuntimeConfig().public.mdc
const generate = computed(
  () =>
    props.id &&
    ((typeof headings?.anchorLinks === 'boolean' &&
      headings?.anchorLinks === true) ||
      (typeof headings?.anchorLinks === 'object' && headings?.anchorLinks?.h2))
)
</script>
