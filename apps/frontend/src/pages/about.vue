<template>
  <div
    class="grid grid-cols-1 divide-y divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 lg:grid-cols-2 lg:divide-x lg:divide-y-0"
  >
    <div class="flex items-center p-8 lg:p-16">
      <div class="space-y-4">
        <BrandEmblem :size="32" class="text-brand-50 hidden md:block" />
        <h1 class="!text-4xl">
          We are <span class="text-brand-50">Blueprint</span>.
        </h1>
        <p class="text-default-font/75">
          <span class="text-default-font">
            Blueprint went from "product idea" to open-source and nonprofit
            extension platform
          </span>
          for Pterodactyl. Over the years, we've made memories, met so many cool
          people and set new standards.
        </p>

        <p class="text-default-font/50 mt-8">
          Pictures taken on a Polaroid Now+ (Gen 3) instant camera at the
          <a href="https://wiki.why2025.org/Village:Blueprint" class="text-link"
            >Blueprint WHY2025 Village</a
          >.
        </p>
      </div>
    </div>

    <div
      class="bg-stripes relative flex min-h-[300px] items-center justify-center overflow-hidden lg:min-h-[350px]"
    >
      <div class="relative h-full w-full">
        <div
          class="absolute right-[-40px] top-4 rotate-12 transform transition-transform duration-300 hover:rotate-6 hover:scale-105"
        >
          <NuxtImg
            src="/img/polaroids/tent.jpg"
            :width="300"
            class="shadow-lg"
          />
        </div>
        <div
          class="absolute left-1/2 top-1/2 hidden -translate-x-1/2 -translate-y-1/2 -rotate-12 transform transition-transform duration-300 hover:-rotate-6 hover:scale-105 sm:block"
        >
          <NuxtImg
            src="/img/polaroids/flag.jpg"
            :width="280"
            class="shadow-lg"
          />
        </div>
        <div
          class="absolute bottom-4 left-[-40px] rotate-6 transform transition-transform duration-300 hover:rotate-12 hover:scale-105"
        >
          <NuxtImg
            src="/img/polaroids/tents.jpg"
            :width="290"
            class="shadow-lg"
          />
        </div>
      </div>
    </div>
  </div>

  <MarketingTestimonials />
</template>
