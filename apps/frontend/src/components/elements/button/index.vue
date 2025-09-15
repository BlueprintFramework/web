<template>
  <button
    :disabled="props.disabled"
    :class="
      props.color == 'danger'
        ? 'border-red-900 bg-red-950 font-bold text-red-400 hover:bg-red-900 hover:text-red-300 disabled:bg-red-950/50 disabled:text-red-400/50'
        : 'hover:text-brand-50 disabled:text-default-font/50 border-neutral-700 bg-neutral-900 hover:bg-neutral-800 disabled:bg-neutral-950'
    "
    class="duration-250 min-w-15 cursor-pointer rounded-xl border p-2 transition-colors disabled:cursor-not-allowed"
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
