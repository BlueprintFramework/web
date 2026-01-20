<template>
  <NuxtLoadingIndicator color="#52A9FF" :height="1" />
  <div class="h-13 block w-full md:hidden">
    <div class="h-15 fixed left-0 top-4 z-50 w-full">
      <div class="container h-full">
        <div
          class="flex h-full flex-row items-center justify-between rounded-full border border-neutral-700 bg-neutral-950 px-3"
        >
          <div class="ms-1">
            <NuxtLink to="/" class="group block sm:hidden">
              <BrandEmblem
                class="group-hover:text-brand-50 transition-colors"
              />
            </NuxtLink>
            <BrandWordmark class="hidden sm:block" />
          </div>
          <div class="flex flex-row">
            <div
              class="flex flex-row divide-x divide-neutral-700 overflow-hidden rounded-full border border-neutral-700 bg-neutral-900"
            >
              <NuxtLink
                v-if="user?.email_pending == null"
                to="/app"
                class="text-default-font/50 hover:text-brand-50 flex-col items-center p-2 transition-colors hover:bg-neutral-800"
                :class="{ 'text-default-font!': route.path == '/app' }"
              >
                <Icon name="memory:apps" :size="20" mode="svg" />
              </NuxtLink>
              <NuxtLink
                to="/app/account"
                class="text-default-font/50 hover:text-brand-50 flex-col items-center p-2 transition-colors hover:bg-neutral-800"
                :class="{
                  'text-default-font!':
                    route.path == '/app/account' ||
                    route.path.startsWith('/app/account/'),
                }"
              >
                <Icon name="memory:account-box" :size="20" mode="svg" />
              </NuxtLink>
              <NuxtLink
                v-if="user?.email_pending == null"
                to="/app/extensions"
                class="text-default-font/50 hover:text-brand-50 flex-col items-center p-2 transition-colors hover:bg-neutral-800"
                :class="{
                  'text-default-font!':
                    route.path == '/app/extensions' ||
                    route.path.startsWith('/app/extensions/'),
                }"
              >
                <Icon name="memory:cube" :size="20" mode="svg" />
              </NuxtLink>
              <NuxtLink
                v-if="user?.email_pending == null"
                to="/app/stats"
                class="text-default-font/50 hover:text-brand-50 flex-col items-center p-2 transition-colors hover:bg-neutral-800"
                :class="{
                  'text-default-font!':
                    route.path == '/app/stats' ||
                    route.path.startsWith('/app/stats/'),
                }"
              >
                <Icon name="memory:chart-bar" :size="20" mode="svg" />
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="flex gap-5">
    <div class="w-17 hidden py-4 pl-4 md:block">
      <div
        class="w-15 parent-focus fixed z-50 flex h-[calc(100%-2rem)] flex-col items-center divide-y divide-neutral-700 overflow-hidden rounded-2xl border border-neutral-700 bg-neutral-950 transition-colors focus-within:divide-neutral-500 focus-within:border-neutral-500"
      >
        <!-- Emblem -->
        <NuxtLink
          to="/"
          class="group flex aspect-square w-full flex-col items-center justify-center outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
          @mousedown.prevent
        >
          <BrandEmblem
            :size="24"
            class="group-hover:text-brand-50 group-focus:text-brand-50 transition-colors"
          />
        </NuxtLink>

        <!-- Links -->
        <div
          class="apply-parent-focus flex w-full flex-col divide-y divide-neutral-700 transition-colors"
        >
          <NuxtLink
            v-if="user?.email_pending == null"
            to="/app"
            class="text-default-font/50 hover:text-brand-50 focus:text-brand-50 flex w-full flex-col items-center py-4 outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
            :class="{ 'text-default-font!': route.path == '/app' }"
            @mousedown.prevent
          >
            <Icon name="memory:apps" :size="24" />
          </NuxtLink>
          <NuxtLink
            to="/app/account"
            class="text-default-font/50 hover:text-brand-50 focus:text-brand-50 flex w-full flex-col items-center py-4 outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
            :class="{
              'text-default-font!':
                route.path == '/app/account' ||
                route.path.startsWith('/app/account/'),
            }"
            @mousedown.prevent
          >
            <Icon name="memory:account-box" :size="24" />
          </NuxtLink>
          <NuxtLink
            v-if="user?.email_pending == null"
            to="/app/extensions"
            class="text-default-font/50 hover:text-brand-50 focus:text-brand-50 flex w-full flex-col items-center py-4 outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
            :class="{
              'text-default-font!':
                route.path == '/app/extensions' ||
                route.path.startsWith('/app/extensions/'),
            }"
            @mousedown.prevent
          >
            <Icon name="memory:cube" :size="24" />
          </NuxtLink>
          <NuxtLink
            v-if="user?.email_pending == null"
            to="/app/stats"
            class="text-default-font/50 hover:text-brand-50 focus:text-brand-50 flex w-full flex-col items-center py-4 outline-0 transition-colors hover:bg-neutral-900 focus:bg-neutral-900"
            :class="{
              'text-default-font!':
                route.path == '/app/stats' ||
                route.path.startsWith('/app/stats/'),
            }"
            @mousedown.prevent
          >
            <Icon name="memory:chart-bar" :size="24" />
          </NuxtLink>
        </div>

        <!-- Logout -->
        <div
          class="bg-stripes flex h-full w-full flex-col justify-end transition-colors"
        >
          <button
            @click="logout"
            class="apply-parent-focus flex cursor-pointer flex-col items-center border-t border-neutral-700 bg-neutral-950 py-4 outline-0 transition-colors hover:bg-red-950 hover:text-red-400 focus:bg-red-950 focus:text-red-400"
            @mousedown.prevent
          >
            <Icon name="memory:logout" :size="24" />
          </button>
        </div>
      </div>
    </div>
    <div class="flex w-full">
      <div class="container space-y-5 pb-6 pt-11 md:pt-6">
        <UiAppVerifyemail v-if="user?.email_pending != null" />
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { user, logout } = useAuth()
const route = useRoute()
</script>

<style scoped>
@reference "~/assets/css/main.css";

.parent-focus:focus-within .apply-parent-focus {
  @apply divide-neutral-500 border-neutral-500;
}
</style>

<style scoped>
.nuxt-loading-indicator {
  top: 0 !important;
}
</style>
