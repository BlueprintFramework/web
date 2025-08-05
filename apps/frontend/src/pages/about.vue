<template>
  <div class="space-y-12 lg:space-y-0">
    <div
      class="grid grid-cols-1 divide-y divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 lg:grid-cols-2 lg:divide-x lg:divide-y-0 lg:rounded-b-none"
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
            for Pterodactyl. Over the years, we've made memories, met so many
            cool people and set new standards.
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
              src="/img/polaroids/shark.jpg"
              :width="300"
              class="shadow-lg"
            />
          </div>
          <div
            class="absolute left-1/2 top-1/2 hidden -translate-x-1/2 -translate-y-1/2 -rotate-12 transform transition-transform duration-300 hover:-rotate-6 hover:scale-105 sm:block"
          >
            <NuxtImg
              src="/img/polaroids/beach.jpg"
              :width="280"
              class="shadow-lg"
            />
          </div>
          <div
            class="absolute bottom-4 left-[-40px] rotate-6 transform transition-transform duration-300 hover:rotate-12 hover:scale-105"
          >
            <NuxtImg
              src="/img/polaroids/station.png"
              :width="290"
              class="shadow-lg"
            />
          </div>
        </div>
      </div>
    </div>

    <div
      class="rounded-3xl border border-neutral-700 lg:rounded-t-none lg:border-t-0"
    >
      <div
        class="grid grid-cols-1 divide-y divide-neutral-700 md:grid-cols-2 md:divide-x md:divide-y-0"
      >
        <div class="space-y-4 p-8 lg:p-16">
          <h1>Impact</h1>
          <p class="text-default-font/75">
            Blueprint has played a vital role in both setting Pterodactyl
            modding standards and holding them up.
          </p>
        </div>
        <div class="space-y-4 p-8 lg:p-16">
          <h1>Community</h1>
          <p class="text-default-font/75">
            Blueprint's community is a mix of developers, system administrators,
            freelancers and hosting providers.
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
