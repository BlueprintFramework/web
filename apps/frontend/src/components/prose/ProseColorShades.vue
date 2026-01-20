<template>
  <div
    class="my-4 flex flex-col gap-[1px] overflow-hidden rounded-2xl border border-neutral-700 bg-neutral-700 max-lg:overflow-x-scroll"
  >
    <div
      v-for="colorFamily in content"
      :key="colorFamily.color"
      class="flex gap-[1px] bg-neutral-700"
    >
      <div
        class="min-w-25 flex items-center justify-center bg-neutral-950 px-4 py-2"
      >
        <span class="text-sm font-medium text-white">{{
          colorFamily.color
        }}</span>
      </div>

      <div
        v-for="hue in colorFamily.hues"
        :key="hue.variant"
        class="max-lg:min-w-25 group relative flex flex-1 flex-col items-center justify-center bg-neutral-950 px-1 py-2 transition-colors hover:bg-neutral-900"
        :style="
          validateRgb(hue.rgb)
            ? `background-color: rgb(${hue.rgb});`
            : 'background-color: rgb(0 0 0);'
        "
      >
        <template v-if="validateRgb(hue.rgb)">
          <span
            class="absolute text-xs font-medium leading-tight opacity-100 transition-opacity duration-200 group-hover:opacity-0"
            :class="
              needsDarkText(hue.rgb) ? 'text-neutral-900' : 'text-default-font'
            "
          >
            {{ hue.variant }}
          </span>

          <span
            class="absolute text-[10px] leading-tight opacity-0 transition-opacity duration-200 group-hover:opacity-75"
            :class="
              needsDarkText(hue.rgb) ? 'text-neutral-800' : 'text-neutral-300'
            "
          >
            {{ hue.rgb }}
          </span>
        </template>

        <template v-else>
          <span class="text-xs text-red-400">×</span>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  content: {
    color: string
    hues: {
      variant: string
      rgb: string
    }[]
  }[]

  class?: string
  style?: string
}>()

function validateRgb(rgb: string): boolean {
  if (!rgb) return false

  const parts = rgb.split(' ')
  if (parts.length !== 3) return false

  const r = parseInt(parts[0] || '0', 10)
  const g = parseInt(parts[1] || '0', 10)
  const b = parseInt(parts[2] || '0', 10)

  if (isNaN(r) || isNaN(g) || isNaN(b)) return false

  if (r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255) return false

  return true
}

function needsDarkText(rgb: string): boolean {
  if (!validateRgb(rgb)) return false

  const parts = rgb.split(' ')
  const r = parseInt(parts[0] || '0', 10)
  const g = parseInt(parts[1] || '0', 10)
  const b = parseInt(parts[2] || '0', 10)

  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
  return luminance > 0.5
}
</script>
