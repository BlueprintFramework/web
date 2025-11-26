<template>
  <h1 :id="props.id" style="font-size: 38px" class="mb-4 mt-16">
    <a
      v-if="generate"
      :href="`#${$props.id}`"
      class="hover:text-brand-50 transition-colors"
    >
      <slot />
    </a>
    <slot v-else />
  </h1>
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
      (typeof headings?.anchorLinks === 'object' && headings?.anchorLinks?.h1))
)
</script>

<style scoped>
* + h1 {
  margin-top: 1.75rem;
}
</style>
