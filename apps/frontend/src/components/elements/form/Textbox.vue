<template>
  <textarea />
</template>

<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    autofocus?: boolean
    cols?: number
    dirname?: string
    disabled?: boolean
    form?: string
    maxlength?: number
    name?: string
    placeholder?: string
    readonly?: boolean
    required?: boolean
    rows?: number
    wrap?: 'hard' | 'soft'
  }>(),
  {
    wrap: 'hard',
  }
)

const modelValue = defineModel<string>()
</script>
