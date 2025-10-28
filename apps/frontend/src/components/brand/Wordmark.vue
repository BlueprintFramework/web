<template>
  <NuxtLink
    v-if="showWordart"
    to="/"
    class="hover:rotate-13 focus:rotate-13 transition-transform duration-300 hover:scale-110"
  >
    <NuxtImg src="/img/blueprint_wordart.png" :height="28" />
  </NuxtLink>
  <div v-else class="w-min">
    <NuxtLink
      to="/"
      class="hover:text-brand-50 focus:text-brand-50 flex items-center justify-between gap-2 outline-0 transition-colors"
      :class="emblemClass"
      @mouseover="emblemHover()"
      @focus="emblemHover()"
    >
      <BrandEmblem />
      <span class="text-xl font-bold">Blueprint</span>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
const today = new Date()
const showWordart = today.getMonth() === 3 && today.getDate() === 1

const emblemClass = ref('')
const emblemAnimating = ref(false)

const emblemHover = () => {
  if (!emblemAnimating.value) {
    emblemAnimating.value = true
    emblemClass.value = 'emblem-animate'
    setTimeout(() => {
      emblemClass.value = ''
      emblemAnimating.value = false
    }, 800)
  }
}
</script>
