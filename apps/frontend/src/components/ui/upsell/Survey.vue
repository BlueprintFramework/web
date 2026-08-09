<template>
  <Transition
    mode="out-in"
    enter-active-class="transition-all duration-300 ease-out"
    enter-from-class="opacity-0 translate-y-4"
    enter-to-class="opacity-100 translate-y-0"
    leave-active-class="transition-all duration-200 ease-in"
    leave-from-class="opacity-100 translate-y-0"
    leave-to-class="opacity-0 translate-y-4"
  >
    <div
      v-if="visible"
      class="w-100 bg-linear-to-br to-brand-800 drop-shadow-neutral-950 fixed bottom-0 right-0 rounded-tl-3xl border-l border-t border-neutral-600 from-neutral-950 drop-shadow-2xl max-md:hidden"
    >
      <div class="relative">
        <NuxtImg
          src="/img/stickers/blu_stretch.png"
          class="-top-15 hover:rotate-20 absolute -left-10 rotate-12 transition-transform hover:scale-105"
          width="150"
        />
        <NuxtImg
          src="/img/stickers/blueprinter.png"
          class="left-23 -top-17 hover:-rotate-20 absolute -rotate-12 transition-transform hover:scale-105"
          width="150"
        />
        <NuxtImg
          src="/img/stickers/shift.png"
          class="left-55 hover:rotate-8 absolute -top-7 rotate-0 transition-transform hover:scale-105"
          width="200"
        />
      </div>

      <div class="space-y-2 p-4 pt-20">
        <p class="text-2xl">
          Tell us what you dislike about Blueprint
          <Icon
            name="pixelarticons:arrow-right"
            size="24"
            class="translate-y-1"
          />
          win stickers*
        </p>

        <p class="">
          We’re curious about your experience with Blueprint and our projects.
        </p>

        <div class="grid grid-rows-2 gap-2 pt-2">
          <NuxtLink
            to="https://forms.blueprint.zip/r/plzfix-aug26"
            class="w-full"
          >
            <ElementsButton
              label="Share your feedback (~1min)"
              class="hover:bg-brand-200! bg-brand-50! border-brand-50! text-neutral-950! w-full"
            />
          </NuxtLink>
          <ElementsButton label="Don't show again" @click="dismiss()" />
        </div>

        <p class="text-sm opacity-50">
          *5 random participants will win a sticker pack on Aug 14. Free &
          optional.
        </p>
      </div>
    </div>
  </Transition>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'

const STORAGE_KEY = 'feedback_aug26'
const visible = ref(false)
let timerId = null

function dismiss() {
  try {
    localStorage.setItem(STORAGE_KEY, '1')
  } catch (e) {}
  visible.value = false
}

onMounted(() => {
  try {
    if (localStorage.getItem(STORAGE_KEY) === '1') return
  } catch (e) {}

  timerId = window.setTimeout(() => {
    visible.value = true
  }, 3000)
})

onBeforeUnmount(() => {
  if (timerId) window.clearTimeout(timerId)
})
</script>
