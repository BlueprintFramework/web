<template>
  <UiNavigation />
  <div class="container">
    <div
      class="grid h-[calc(100vh_-_var(--nav-offset))] grid-cols-2 divide-x divide-neutral-700 overflow-hidden border-x border-neutral-700 bg-neutral-950"
    >
      <div
        class="relative flex h-full items-center bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:30px_30px]"
      >
        <div
          class="absolute inset-0 z-0 bg-gradient-to-b from-transparent via-neutral-950 to-transparent"
        />
        <div class="relative z-10 w-full bg-neutral-950">
          <slot />
        </div>
      </div>
      <div class="bg-[black]">
        <NuxtImg
          src="/img/website.jpeg"
          class="h-[calc(100vh_-_var(--nav-offset))] w-full object-cover"
        />
      </div>
    </div>
  </div>
</template>
