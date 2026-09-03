<template>
  <NuxtLink
    v-if="showWordart"
    to="/"
    class="hover:rotate-13 focus:rotate-13 transition-transform duration-300 hover:scale-110"
    @mousedown.prevent
  >
    <img src="/img/blueprint_wordart_28.png" :height="28" />
  </NuxtLink>
  <div v-else class="w-min">
    <NuxtLink
      to="/"
      class="hover:text-brand-50 focus:text-brand-50 flex items-center justify-between gap-2 outline-0 transition-colors"
      :class="emblemClass"
      @mouseover="emblemHover()"
      @focus="emblemHover()"
      @mousedown.prevent
    >
      <BrandEmblem />
      <span class="text-xl font-bold">Blueprint</span>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
const emblemClass = ref('')
const emblemAnimating = ref(false)
const showWordart = ref(false)

onMounted(() => {
  const today = new Date()
  if (today.getMonth() === 3 && today.getDate() === 1) {
    showWordart.value = true
  }
})

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
