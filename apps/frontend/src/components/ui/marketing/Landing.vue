<template>
  <div class="bg-neutral-950">
    <div class="bg-stripes">
      <div
        class="max-md:bg-linear-to-tl from-brand-600 border-b border-neutral-700 to-neutral-950 to-50%"
      >
        <div class="container">
          <div
            class="xl:max-h-140 xl:min-h-120 md:bg-linear-to-tl from-brand-600 max-md:min-h-100 max-xl:min-h-180 relative grid h-[40vh] items-center overflow-hidden border-neutral-700 to-neutral-950 to-50% max-xl:grid-rows-2 max-md:grid-rows-1 md:border-x md:pt-12 lg:pt-6 xl:grid-cols-2 xl:pt-0"
          >
            <div class="space-y-7 max-xl:pb-6 max-md:pt-2 md:p-12 xl:pe-6">
              <p class="md:display h1">
                The Pterodactyl® community builds on
                <span
                  class="text-brand-50 inline-flex items-center gap-2 md:gap-4"
                >
                  <BrandEmblem :size="22" class="md:hidden" />
                  <BrandEmblem :size="44" class="hidden md:block" />
                  Blueprint
                </span>
              </p>
              <p class="text-default-font/70">
                <span class="text-default-font">
                  Pterodactyl's favorite modding community.
                </span>
                Develop, collaborate and install extensions with the extension
                platform that puts you first.
              </p>
              <div class="flex items-center gap-4">
                <a href="/guides/admin/install" tabindex="-1">
                  <button
                    class="bg-brand-50 hover:bg-brand-200 flex cursor-pointer items-center gap-1 rounded-full px-3 py-1.5 font-bold text-neutral-950 transition-colors"
                  >
                    <span> Install now </span>
                    <Icon name="memory:chevron-right" :size="22" />
                  </button>
                </a>
                <a
                  href="https://github.com/blueprintframework/framework"
                  tabindex="-1"
                  class="hidden md:block"
                >
                  <button
                    class="text-brand-50 flex cursor-pointer items-center gap-2 rounded-full px-3 py-1.5 font-bold transition-colors hover:bg-white/10"
                  >
                    <SvgGithub :size="22" />
                    <span> Star on GitHub </span>
                  </button>
                </a>
              </div>
            </div>
            <div class="hidden h-full overflow-hidden md:block xl:ps-6">
              <div class="perspective-distant relative h-full w-full">
                <NuxtImg
                  src="/img/pterodactyl2.jpeg"
                  :width="800"
                  class="xl:-bottom-1/12 max-xl:top-1/12 max-xl:rotate-x-7 xl:rotate-x-12 -rotate-y-15 xl:left-1/12 absolute rounded-2xl border-2 border-neutral-700 max-lg:left-6 lg:left-14"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
