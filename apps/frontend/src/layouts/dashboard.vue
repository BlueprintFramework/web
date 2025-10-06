<template>
  <div v-if="!isDesktop" class="h-13 w-full">
    <div
      class="h-13 fixed left-0 top-0 z-50 w-full border-b border-neutral-700 bg-neutral-950"
    >
      <div class="container h-full">
        <div class="flex h-full flex-row items-center justify-between">
          <NuxtLink to="/" v-if="isXs" class="group">
            <BrandEmblem class="group-hover:text-brand-50 transition-colors" />
          </NuxtLink>
          <BrandWordmark v-else />
          <div class="flex flex-row">
            <div
              class="flex flex-row divide-x divide-neutral-700 overflow-hidden rounded-full border border-neutral-700 bg-neutral-900"
            >
              <NuxtLink
                v-if="user?.email_pending == null"
                to="/app"
                class="text-default-font/50 hover:text-brand-50 flex-col items-center p-2 transition-colors hover:bg-neutral-800"
                :class="{ '!text-default-font': route.path == '/app' }"
              >
                <Icon name="memory:apps" :size="20" mode="svg" />
              </NuxtLink>
              <NuxtLink
                to="/app/account"
                class="text-default-font/50 hover:text-brand-50 flex-col items-center p-2 transition-colors hover:bg-neutral-800"
                :class="{
                  '!text-default-font':
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
                  '!text-default-font':
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
                  '!text-default-font':
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
    <div v-if="isDesktop" class="w-17 py-4 pl-4">
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
            v-if="user?.email_pending == null"
            to="/app"
            class="text-default-font/50 hover:text-brand-50 flex w-full flex-col items-center py-4 transition-colors hover:bg-neutral-900"
            :class="{ '!text-default-font': route.path == '/app' }"
          >
            <Icon name="memory:apps" :size="24" />
          </NuxtLink>
          <NuxtLink
            to="/app/account"
            class="text-default-font/50 hover:text-brand-50 flex w-full flex-col items-center py-4 transition-colors hover:bg-neutral-900"
            :class="{
              '!text-default-font':
                route.path == '/app/account' ||
                route.path.startsWith('/app/account/'),
            }"
          >
            <Icon name="memory:account-box" :size="24" />
          </NuxtLink>
          <NuxtLink
            v-if="user?.email_pending == null"
            to="/app/extensions"
            class="text-default-font/50 hover:text-brand-50 flex w-full flex-col items-center py-4 transition-colors hover:bg-neutral-900"
            :class="{
              '!text-default-font':
                route.path == '/app/extensions' ||
                route.path.startsWith('/app/extensions/'),
            }"
          >
            <Icon name="memory:cube" :size="24" />
          </NuxtLink>
          <NuxtLink
            v-if="user?.email_pending == null"
            to="/app/stats"
            class="text-default-font/50 hover:text-brand-50 flex w-full flex-col items-center py-4 transition-colors hover:bg-neutral-900"
            :class="{
              '!text-default-font':
                route.path == '/app/stats' ||
                route.path.startsWith('/app/stats/'),
            }"
          >
            <Icon name="memory:chart-bar" :size="24" />
          </NuxtLink>
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
      <div class="container space-y-5 py-6">
        <UiAppVerifyemail v-if="user?.email_pending != null" />
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMediaQuery } from '@vueuse/core'

const { user, logout } = useAuth()
const route = useRoute()
const isDesktop = useMediaQuery('(min-width: 768px)')
const isXs = useMediaQuery('(max-width: 320px)')
</script>
