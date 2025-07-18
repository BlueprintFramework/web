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
          <div
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

      <div
        @click="mobileNavigation = false"
        class="absolute pt-10 opacity-0 transition-opacity md:hidden"
        :class="{
          'opacity-100': mobileNavigation,
        }"
      >
        <UiNavigationMobilelink
          to="/browse"
          label="Extensions"
          :delay="100"
          :visible="mobileNavigation"
        />
        <UiNavigationMobilelink
          to="/guides"
          label="Guides"
          :delay="150"
          :visible="mobileNavigation"
        />
        <UiNavigationMobilelink
          to="/docs"
          label="Documentation"
          :delay="200"
          :visible="mobileNavigation"
        />

        <div class="py-5" />

        <UiNavigationMobilelink
          to="/auth"
          label="Log in"
          :delay="250"
          :visible="mobileNavigation"
        />
        <UiNavigationMobilelink
          to="/auth/register"
          label="Sign up"
          :delay="300"
          :visible="mobileNavigation"
        />
      </div>
    </div>

    <div
      class="bg-linear-to-r relative top-0 h-[1px] w-full from-neutral-800 via-neutral-500 to-neutral-800 transition-all duration-500 md:static"
      :class="{ 'top-[calc(100dvh_-_var(--nav-offset))]': mobileNavigation }"
    />
  </nav>
  <div class="h-13"></div>
</template>

<style>
:root {
  --nav-offset: 3.25rem;
}
</style>

<script setup lang="ts">
const mobileNavigation = ref(false)
</script>
