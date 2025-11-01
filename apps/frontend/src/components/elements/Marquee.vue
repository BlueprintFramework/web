<template>
  <div
    ref="marqueeContainer"
    class="relative w-full overflow-hidden"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
    @mousedown="handleMouseDown"
    @touchstart="handleTouchStart"
    @click.capture="handleClick"
  >
    <div
      ref="marqueeTrack"
      class="flex w-max"
      :style="{
        transform: `translateX(${translateX}px)`,
        cursor: isDragging ? 'grabbing' : 'grab',
        touchAction: 'pan-y',
      }"
    >
      <div class="flex">
        <slot />
      </div>
      <div class="flex">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'

interface Props {
  direction?: 'left' | 'right'
  speed?: number // pixels per second
  pauseOnHover?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  direction: 'left',
  speed: 20, // default 20 pixels per second
  pauseOnHover: true,
})

const marqueeContainer = useTemplateRef('marqueeContainer')
const marqueeTrack = useTemplateRef('marqueeTrack')

const translateX = ref(0)
const isHovered = ref(false)
const isDragging = ref(false)

// Convert pixels per second to pixels per frame (assuming 60fps)
const speedPerFrame = computed(() => {
  const pixelsPerFrame = props.speed / 60
  return props.direction === 'left' ? pixelsPerFrame : -pixelsPerFrame
})

let animationFrameId: number | null = null
let contentWidth = 0

// Drag state
const dragStartX = ref(0)
const dragStartTranslateX = ref(0)

// Momentum/flick state
const velocity = ref(0)
const deceleration = 0.95 // friction factor (0.95 = 5% slowdown per frame)
const minVelocity = 0.1 // stop when velocity is below this threshold

// Track mouse positions for velocity calculation
let lastMouseX = 0
let lastMouseTime = 0
let hasDragged = false

const normalizePosition = () => {
  if (contentWidth === 0) return

  // Keep position within bounds (0 to -contentWidth)
  // This allows seamless looping in both directions
  if (translateX.value > 0) {
    translateX.value = translateX.value - contentWidth
  } else if (translateX.value <= -contentWidth) {
    translateX.value = translateX.value + contentWidth
  }
}

const handleMouseEnter = () => {
  isHovered.value = true
}

const handleMouseLeave = () => {
  isHovered.value = false
}

const handleClick = (e: MouseEvent) => {
  // Prevent clicks on links if user was dragging
  if (hasDragged) {
    e.preventDefault()
    e.stopPropagation()
  }
}

const handleMouseDown = (e: MouseEvent) => {
  isDragging.value = true
  dragStartX.value = e.clientX
  dragStartTranslateX.value = translateX.value
  hasDragged = false

  // Reset velocity when starting a new drag
  velocity.value = 0

  // Initialize velocity tracking
  lastMouseX = e.clientX
  lastMouseTime = Date.now()

  document.addEventListener('mousemove', handleMouseMove)
  document.addEventListener('mouseup', handleMouseUp)

  // Prevent text selection and link behavior
  e.preventDefault()
}

const handleMouseMove = (e: MouseEvent) => {
  if (!isDragging.value) return

  const deltaX = e.clientX - dragStartX.value
  translateX.value = dragStartTranslateX.value + deltaX

  // Track if user has actually dragged (moved more than 5px)
  if (Math.abs(deltaX) > 5) {
    hasDragged = true
  }

  // Calculate velocity for flick/momentum
  const currentTime = Date.now()
  const timeDelta = currentTime - lastMouseTime

  if (timeDelta > 0) {
    const moveDelta = e.clientX - lastMouseX
    velocity.value = (moveDelta / timeDelta) * 16 // normalize to ~60fps
  }

  lastMouseX = e.clientX
  lastMouseTime = currentTime

  // Normalize position to keep within bounds for seamless looping
  normalizePosition()
}

const handleMouseUp = () => {
  isDragging.value = false
  normalizePosition()
  document.removeEventListener('mousemove', handleMouseMove)
  document.removeEventListener('mouseup', handleMouseUp)
}

// Touch event handlers for mobile
const handleTouchStart = (e: TouchEvent) => {
  if (e.touches.length !== 1) return

  isDragging.value = true
  const touch = e.touches[0]
  dragStartX.value = touch.clientX
  dragStartTranslateX.value = translateX.value
  hasDragged = false

  // Reset velocity when starting a new drag
  velocity.value = 0

  // Initialize velocity tracking
  lastMouseX = touch.clientX
  lastMouseTime = Date.now()

  document.addEventListener('touchmove', handleTouchMove, { passive: false })
  document.addEventListener('touchend', handleTouchEnd)
  document.addEventListener('touchcancel', handleTouchEnd)
}

const handleTouchMove = (e: TouchEvent) => {
  if (!isDragging.value || e.touches.length !== 1) return

  const touch = e.touches[0]
  const deltaX = touch.clientX - dragStartX.value
  translateX.value = dragStartTranslateX.value + deltaX

  // Track if user has actually dragged (moved more than 5px)
  if (Math.abs(deltaX) > 5) {
    hasDragged = true
    // Prevent scrolling when dragging
    e.preventDefault()
  }

  // Calculate velocity for flick/momentum
  const currentTime = Date.now()
  const timeDelta = currentTime - lastMouseTime

  if (timeDelta > 0) {
    const moveDelta = touch.clientX - lastMouseX
    velocity.value = (moveDelta / timeDelta) * 16 // normalize to ~60fps
  }

  lastMouseX = touch.clientX
  lastMouseTime = currentTime

  // Normalize position to keep within bounds for seamless looping
  normalizePosition()
}

const handleTouchEnd = () => {
  isDragging.value = false
  normalizePosition()
  document.removeEventListener('touchmove', handleTouchMove)
  document.removeEventListener('touchend', handleTouchEnd)
  document.removeEventListener('touchcancel', handleTouchEnd)
}

const animate = () => {
  if (!isDragging.value) {
    // Apply momentum if there's any velocity from flicking (even when hovering)
    if (Math.abs(velocity.value) > minVelocity) {
      translateX.value += velocity.value
      velocity.value *= deceleration // apply friction
      normalizePosition()
    } else if (!isHovered.value || !props.pauseOnHover) {
      // Default auto-scroll when no momentum and either:
      // - not hovering, OR
      // - pauseOnHover is disabled
      velocity.value = 0
      translateX.value -= speedPerFrame.value
      normalizePosition()
    }
  }

  animationFrameId = requestAnimationFrame(animate)
}

onMounted(() => {
  // Calculate the width of the content for seamless looping
  if (marqueeTrack.value) {
    const firstChild = marqueeTrack.value.firstElementChild as HTMLElement
    if (firstChild) {
      contentWidth = firstChild.offsetWidth
    }
  }

  // Start animation
  animate()
})

onUnmounted(() => {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId)
  }
  // Clean up mouse event listeners
  document.removeEventListener('mousemove', handleMouseMove)
  document.removeEventListener('mouseup', handleMouseUp)
  // Clean up touch event listeners
  document.removeEventListener('touchmove', handleTouchMove)
  document.removeEventListener('touchend', handleTouchEnd)
  document.removeEventListener('touchcancel', handleTouchEnd)
})
</script>

<style scoped>
/* Prevent text selection while dragging */
.flex {
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}
</style>
