<template>
  <nav class="h-13 fixed z-10 w-full bg-neutral-950">
    <div class="container py-3">
      <div class="relative flex items-center justify-between">
        <NuxtLink
          to="/"
          class="hover:text-brand-50 flex items-center justify-between space-x-2 transition-colors"
          :class="emblemClass"
          @mouseover="emblemHover()"
        >
          <BrandEmblem />
          <span class="text-xl font-bold">Blueprint</span>
        </NuxtLink>
        <div class="flex items-center justify-between space-x-4">
          <NuxtLink to="/browse"> Extensions </NuxtLink>
          <NuxtLink to="/docs"> Documentation </NuxtLink>
        </div>
      </div>
    </div>
    <div
      class="bg-linear-to-r h-[1px] w-full from-neutral-800 via-neutral-500 to-neutral-800"
    ></div>
  </nav>
  <div class="h-13"></div>
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
