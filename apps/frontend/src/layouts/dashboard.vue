<template>
  <div class="flex">
    <div class="w-21 py-4 pl-4">
      <div
        class="w-15 fixed z-50 flex h-[calc(100%_-_2rem)] flex-col items-center divide-y divide-neutral-700 overflow-hidden rounded-2xl border border-neutral-700 bg-neutral-950"
      >
        <!-- Emblem -->
        <NuxtLink
          to="/"
          class="group flex aspect-square w-full flex-col items-center justify-center transition-colors hover:bg-neutral-900"
        >
          <BrandEmblem
            :size="24"
            class="group-hover:text-brand-50 transition-colors"
          />
        </NuxtLink>

        <!-- Links -->
        <div class="flex w-full flex-col divide-y divide-neutral-700">
          <NuxtLink
            to="/app"
            class="text-default-font/50 hover:text-brand-50 flex w-full flex-col items-center py-4 transition-colors hover:bg-neutral-900"
            :class="{ '!text-default-font': route.path == '/app' }"
          >
            <Icon name="memory:apps" :size="24" />
          </NuxtLink>
          <div
            class="text-default-font/50 hover:text-brand-50 flex w-full flex-col items-center py-4 transition-colors hover:bg-neutral-900"
          >
            <Icon name="memory:account-box" :size="24" />
          </div>
          <div
            class="text-default-font/50 hover:text-brand-50 flex w-full flex-col items-center py-4 transition-colors hover:bg-neutral-900"
          >
            <Icon name="memory:cube" :size="24" />
          </div>
          <div
            class="text-default-font/50 hover:text-brand-50 flex w-full flex-col items-center py-4 transition-colors hover:bg-neutral-900"
          >
            <Icon name="memory:chart-bar" :size="24" />
          </div>
        </div>

        <!-- Logout -->
        <div class="bg-stripes flex h-full w-full flex-col justify-end">
          <button
            @click="logout"
            class="flex cursor-pointer flex-col items-center border-t border-neutral-700 bg-neutral-950 py-4 transition-colors hover:bg-red-950 hover:text-red-400"
          >
            <Icon name="memory:logout" :size="24" />
          </button>
        </div>
      </div>
    </div>
    <div class="flex w-full">
      <div class="container space-y-6 py-6">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { user, logout } = useAuth()
const route = useRoute()
</script>
