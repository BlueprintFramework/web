<template>
  <span class="display mb-12 mt-4 block xl:text-justify">
    <span class="text-gray-500">The</span>
    industry-leading modding platform
    <span class="text-gray-500">for the Pterodactyl panel.</span>
  </span>

  <div class="grid grid-cols-3">
    <div class="rounded-s-4xl border border-neutral-700 bg-neutral-950">
      <div
        class="relative flex items-center justify-between border-b border-neutral-700 p-4"
      >
        <h2>Extension ecosystem</h2>
        <Icon name="memory:cube" class="text-3xl" mode="svg" />
      </div>
      <div class="p-4">
        <span class="body">
          <span class="opacity-75">
            Pterodactyl themes, plugin installers, player managers, admin tools
            and much more.
          </span>
          There's a Blueprint extension for that.
        </span>
      </div>
    </div>

    <div class="border-y border-neutral-700 bg-neutral-950">
      <div
        class="relative flex items-center justify-between border-b border-neutral-700 p-4"
      >
        <h2>Beginner and developer-friendly</h2>
        <Icon name="memory:application-code" class="text-3xl" mode="svg" />
      </div>
      <div class="p-4">
        <span class="body">
          <span class="opacity-75">
            Our mission is to make Pterodactyl modifications easier for
            everyone.
          </span>
          Thousands of administrators and developers call Blueprint home.
        </span>
      </div>
    </div>

    <div class="rounded-e-4xl border border-neutral-700 bg-neutral-950">
      <div
        class="relative flex items-center justify-between border-b border-neutral-700 p-4"
      >
        <h2>Save time</h2>
        <Icon name="memory:time-sand" class="text-3xl" mode="svg" />
      </div>
      <div class="p-4">
        <span class="body">
          <span class="opacity-75">
            You don't have an infinite supply of time on your hands, do you?
          </span>
          Blueprint saves a huge amount of time
          <span class="opacity-75">
            for both developers and system administrators.
          </span>
        </span>
      </div>
    </div>
  </div>

  <div
    class="top-13 absolute inset-0 -z-10 h-[50vh] w-full bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:30px_30px]"
  >
    <div class="bg-linear-to-b h-full w-full from-transparent to-neutral-950" />
  </div>
</template>
