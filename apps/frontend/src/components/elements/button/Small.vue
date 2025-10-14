<template>
  <button
    :disabled="props.disabled"
    :class="
      `${props.class} ` +
      (props.color == 'danger'
        ? 'border-red-900 bg-red-950 font-bold text-red-400 hover:bg-red-900 hover:text-red-300 disabled:bg-red-950/50 disabled:text-red-400/50'
        : 'hover:text-brand-50 disabled:text-default-font/50 border-neutral-700 bg-neutral-900 hover:bg-neutral-800 disabled:bg-neutral-950')
    "
    class="flex cursor-pointer items-center gap-1 rounded-full border px-2.5 py-0.5 text-sm transition-colors disabled:cursor-not-allowed"
  >
    <template v-if="$slots.default">
      <slot />
    </template>
    <template v-else-if="props.label">
      {{ props.label }}
    </template>
  </button>
</template>

<script setup lang="ts">
interface Props {
  class?: any
  label?: string
  disabled?: boolean
  color?: 'default' | 'danger'
  type?: 'button' | 'submit' | 'reset'
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  color: 'default',
})
</script>
