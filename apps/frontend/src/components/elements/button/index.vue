<template>
  <button
    :disabled="props.disabled"
    :class="
      `${props.class} ` +
      (props.color == 'danger'
        ? 'border-red-900 bg-red-950 font-bold disabled:bg-red-950/50 disabled:text-red-400/50 ' +
          (active
            ? '!bg-red-900 !text-red-300'
            : 'text-red-400 hover:bg-red-900 hover:text-red-300')
        : 'disabled:text-default-font/50 border-neutral-700 bg-neutral-900 disabled:bg-neutral-950 ' +
          (active
            ? '!text-brand-50 !border-blue-800 !bg-blue-950'
            : 'hover:text-brand-50 hover:bg-neutral-800'))
    "
    class="duration-250 min-w-20 cursor-pointer rounded-2xl border px-3 py-2 transition-colors disabled:cursor-not-allowed"
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
  active?: boolean
  disabled?: boolean
  color?: 'default' | 'danger'
  type?: 'button' | 'submit' | 'reset'
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  color: 'default',
  active: false,
})
</script>
