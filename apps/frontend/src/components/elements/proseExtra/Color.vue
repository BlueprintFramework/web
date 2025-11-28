<template>
  <div
    v-if="color"
    class="group overflow-hidden bg-neutral-950 p-3 transition-colors hover:bg-neutral-900"
  >
    <div
      class="flex h-10 flex-col items-center justify-center rounded-xl border border-neutral-700 transition-colors group-hover:!bg-transparent"
      :style="`background-color: rgb(${color[0]} ${color[1]} ${color[2]});`"
    >
      <span class="opacity-0 transition-opacity group-hover:opacity-100">
        {{ `${color[0]} ${color[1]} ${color[2]}` }}
      </span>
    </div>
    <div class="pt-3">
      <span>{{ props.name }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  name?: string
  rgb: string

  // Disable class and style props
  class?: string
  style?: string
}>()

const color = computed(() => {
  if (!props.rgb) return false

  const parts = props.rgb.split(' ')
  if (parts.length !== 3) {
    return false
  }

  const r = parseInt(parts[0] || '0', 10)
  const g = parseInt(parts[1] || '0', 10)
  const b = parseInt(parts[2] || '0', 10)

  if (isNaN(r) || isNaN(g) || isNaN(b)) {
    return false
  }

  if (r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255) {
    return false
  }

  return [r, g, b]
})
</script>
