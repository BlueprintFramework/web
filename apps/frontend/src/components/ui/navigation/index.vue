<template>
  <nav
    class="h-13 md:max-h-13 fixed z-50 w-full bg-neutral-950 transition-all duration-500"
    :class="{
      'h-full': mobileNavigation,
    }"
  >
    <div class="container py-3">
      <div class="relative flex items-center justify-between">
        <BrandWordmark @click="mobileNavigation = false" />
        <div class="hidden items-center justify-between gap-3 text-sm md:flex">
          <UiNavigationLink to="/browse" label="Extensions" />
          <UiNavigationLink to="/guides" label="Guides" />
          <UiNavigationLink to="/docs" label="Documentation" />
          <NuxtLink
            to="https://hcb.hackclub.com/donations/start/blueprint"
            target="_blank"
            class="hover:text-brand-50 flex min-h-[26px] items-center gap-1 overflow-hidden rounded-md border border-neutral-700 bg-neutral-900 px-2.5 py-0.5 transition-colors hover:bg-neutral-800"
          >
            <span>Donate</span>
            <Icon name="memory:arrow-top-right" mode="svg" />
          </NuxtLink>
          <client-only>
            <div
              v-if="isAuthenticated"
              class="flex items-center divide-x divide-neutral-700 overflow-hidden rounded-md border border-neutral-700 bg-neutral-800"
            >
              <NuxtLink
                to="/app"
                class="hover:text-brand-50 flex min-h-[26px] items-center gap-1 px-2.5 py-0.5 transition-colors"
              >
                <Icon name="memory:account" />
                <span> {{ user?.name }} </span>
                <span v-if="user?.admin" class="text-default-font/60">
                  (admin)
                </span>
              </NuxtLink>
              <NuxtLink
                @click="logout"
                class="flex min-h-[26px] cursor-pointer items-center bg-neutral-900 px-2.5 py-0.5 transition-colors hover:bg-red-950 hover:text-red-400"
              >
                <Icon name="memory:logout" mode="svg" />
              </NuxtLink>
            </div>
            <div
              v-else
              class="flex items-center divide-x divide-neutral-700 overflow-hidden rounded-md border border-neutral-700 bg-neutral-800"
            >
              <NuxtLink
                to="/auth"
                class="hover:text-brand-50 bg-neutral-900 px-2.5 py-0.5 transition-colors hover:bg-neutral-800"
              >
                <span>Log in</span>
              </NuxtLink>
              <NuxtLink
                to="/auth/register"
                class="hover:text-brand-50 flex items-center gap-1 px-2.5 py-0.5 transition-colors"
              >
                <span>Sign up</span>
                <Icon name="memory:chevron-right" />
              </NuxtLink>
            </div>
          </client-only>
        </div>
        <button
          @click="mobileNavigation = !mobileNavigation"
          alt="Toggle navigation menu"
          class="block md:hidden"
        >
          <Icon
            :name="
              mobileNavigation ? 'pixelarticons:close' : 'pixelarticons:menu'
            "
            :size="24"
            mode="svg"
          />
        </button>
      </div>

      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0 translate-y-4"
        enter-to-class="opacity-100 translate-y-0"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100 translate-y-0"
        leave-to-class="opacity-0 translate-y-4"
      >
        <div
          v-show="mobileNavigation"
          @click="mobileNavigation = false"
          class="absolute pt-10 md:hidden"
        >
          <UiNavigationMobilelink
            to="/browse"
            label="Extensions"
            :visible="mobileNavigation"
          />
          <UiNavigationMobilelink
            to="/guides"
            label="Guides"
            :visible="mobileNavigation"
          />
          <UiNavigationMobilelink
            to="/docs"
            label="Documentation"
            :visible="mobileNavigation"
          />
          <UiNavigationMobilelink
            to="https://hcb.hackclub.com/donations/start/blueprint"
            target="_blank"
            label="Donate"
            :visible="mobileNavigation"
          />
          <div class="py-5" />
          <client-only>
            <div
              v-if="isAuthenticated"
              class="w-[calc(100vw_-_2rem)] max-w-80 divide-y divide-neutral-700 rounded-2xl border border-neutral-700"
            >
              <div class="flex items-center gap-1.5 p-2 font-bold">
                <Icon name="memory:account" />
                <span class="truncate"> {{ user?.name }} </span>
                <span
                  v-if="user?.admin"
                  class="text-default-font/60 font-normal"
                >
                  (admin)
                </span>
              </div>
              <div
                class="flex flex-col gap-2 p-2 opacity-0 transition-opacity duration-500"
                :class="mobileNavigation ? 'opacity-100' : ''"
              >
                <NuxtLink
                  to="/app"
                  class="hover:text-brand-50 block w-full text-start transition-colors"
                  :class="
                    route.path == '/app'
                      ? 'text-default-font'
                      : 'text-default-font/60'
                  "
                >
                  <span>Dashboard</span>
                </NuxtLink>
                <NuxtLink
                  @click="logout"
                  class="text-default-font/60 block w-full cursor-pointer text-start transition-colors hover:text-red-400"
                >
                  <span>Logout</span>
                </NuxtLink>
              </div>
            </div>
            <template v-else>
              <UiNavigationMobilelink
                to="/auth"
                label="Log in"
                :visible="mobileNavigation"
              />
              <UiNavigationMobilelink
                to="/auth/register"
                label="Sign up"
                :visible="mobileNavigation"
              />
            </template>
          </client-only>
        </div>
      </Transition>
    </div>
    <div
      class="bg-linear-to-r relative top-0 h-[1px] w-full from-neutral-800 via-neutral-500 to-neutral-800 transition-all duration-500 md:static"
      :class="{
        'top-[calc(100dvh_-_var(--nav-offset))]': mobileNavigation,
      }"
    />
  </nav>
  <div class="h-13"></div>
</template>
<script setup lang="ts">
const { isAuthenticated, user, logout } = useAuth()
const route = useRoute()

const mobileNavigation = ref(false)

watch(mobileNavigation, (isOpen) => {
  if (isOpen) {
    document.body.style.overflow = 'hidden'
  } else {
    document.body.style.overflow = ''
  }
})
</script>
