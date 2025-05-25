<template>
    <nav class="bg-black shadow">
      <div class="container mx-auto px-4">
        <div class="flex justify-between items-center py-4">
          <div class="flex items-center">
            <BrandEmblem class="mr-2" />
            <div class="flex items-center">
              <span class="text-white text-xl font-semibold">Blueprint</span>
            </div>
          </div>
          <div class="hidden md:flex space-x-4">
            <NuxtLink to="/" class="text-white transition-transform duration-200 hover:scale-110">Home</NuxtLink>
            <NuxtLink to="/Browse" class="text-white transition-transform duration-200 hover:scale-110">Browse</NuxtLink>
            <NuxtLink to="/Docs" class="text-white transition-transform duration-200 hover:scale-110">Docs</NuxtLink>
          </div>
          <button @click="toggleMenu" class="md:hidden text-white focus:outline-none">
            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
        </div>
        
        <!-- Mobile menu -->
        <div v-if="isMenuOpen" class="md:hidden">
          <div class="px-2 pt-2 pb-3 space-y-1 sm:px-3">
            <NuxtLink to="/" class="text-white block px-3 py-2 rounded-md text-base font-medium transition-transform duration-200 hover:scale-105">Home</NuxtLink>
            <NuxtLink to="/Browse" class="text-white block px-3 py-2 rounded-md text-base font-medium transition-transform duration-200 hover:scale-105">Browse</NuxtLink>
            <NuxtLink to="/Docs" class="text-white block px-3 py-2 rounded-md text-base font-medium transition-transform duration-200 hover:scale-105">Docs</NuxtLink>
          </div>
        </div>
      </div>
    </nav>
  </template>
  
  <script setup lang="ts">
  const isMenuOpen = ref(false);
  
  const toggleMenu = () => {
    isMenuOpen.value = !isMenuOpen.value;
  };
  </script>