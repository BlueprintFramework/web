<template>
  <input
    :autocomplete="props.autocomplete ? 'on' : 'off'"
    :autofocus="props.autofocus"
    :checked="props.checked"
    :type="props.type"
  />
</template>

<script setup lang="ts">
const props = defineProps<{
  accept?: string
  alt?: string
  autocomplete?: boolean
  autofocus?: boolean
  checked?: boolean
  disabled?: boolean
  type:
    | 'button'
    | 'checkbox'
    | 'color'
    | 'date'
    | 'datetime-local'
    | 'email'
    | 'file'
    | 'hidden'
    | 'image'
    | 'month'
    | 'number'
    | 'password'
    | 'radio'
    | 'range'
    | 'reset'
    | 'search'
    | 'submit'
    | 'tel'
    | 'text'
    | 'time'
    | 'url'
    | 'week'
  value?: string
}>()
</script>
