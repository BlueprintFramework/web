<template>
  <canvas ref="canvas"></canvas>
</template>

<script setup lang="ts">
import QRCode from 'qrcode'

const props = defineProps<{
  value: string
  size?: number
}>()

const canvas = useTemplateRef('canvas')

onMounted(() => {
  generateQR()
})

watch(() => [props.value, props.size], generateQR)

async function generateQR() {
  if (!canvas.value) return

  await QRCode.toCanvas(canvas.value, props.value, {
    width: props.size || 200,
    margin: 1,
  })
}
</script>
