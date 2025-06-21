<template>
  <div class="w-min">
    <NuxtLink
      to="/"
      class="hover:text-brand-50 flex items-center justify-between gap-2 transition-colors"
      :class="emblemClass"
      @mouseover="emblemHover()"
    >
      <BrandEmblem />
      <span class="text-xl font-bold">Blueprint</span>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
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
