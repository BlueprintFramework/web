<template>
  <div
    :class="[
      `${props.class} space-y-2 rounded-2xl border p-4`,
      props.type == 'default' || !props.type
        ? 'border-neutral-700 bg-neutral-950'
        : '',
      props.type == 'danger' ? 'border-red-800 bg-red-950 text-red-400' : '',
    ]"
  >
    <div v-if="props.icon" class="flex items-center gap-2">
      <Icon
        :name="customIcon || defaults.icon[props.type ?? 'default']"
        :size="24"
        class="min-w-6"
        type="svg"
      />
      <slot />
    </div>
    <slot v-else />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  class?: string
  type?: 'default' | 'danger'
  icon?: string | boolean
}>()

const customIcon = computed(() => {
  if (typeof props.icon === 'string') {
    return props.icon
  } else {
    return null
  }
})

const defaults = {
  icon: {
    default: 'pixelarticons:info-box',
    danger: 'pixelarticons:warning-box',
  },
}
</script>
