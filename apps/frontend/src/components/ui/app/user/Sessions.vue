<template>
  <div
    class="grid grid-cols-2 divide-x divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700"
  >
    <div class="flex min-h-60 flex-col divide-y divide-neutral-700">
      <div class="space-y-2 p-4">
        <h2>Devices</h2>
        <p>
          This is a list of devices that you are currently signed into. You can
          deauthorize devices by selecting them.
        </p>
      </div>
      <div class="bg-stripes h-full" />
      <div class="flex items-center justify-between">
        <button
          class="disabled:text-default-font/60 hover:not-disabled:text-brand-50 hover:not-disabled:bg-neutral-900 border-r border-neutral-700 p-4 transition-colors disabled:cursor-not-allowed"
          :disabled="page > 1 ? false : true"
        >
          Previous page
        </button>
        <div class="p-4">
          <span>
            Showing {{ pageFirstSession }}-{{ pageLastSession }}
            of
            {{ data?.sessions.total }}
          </span>
        </div>
        <button
          class="disabled:text-default-font/60 hover:not-disabled:text-brand-50 hover:not-disabled:bg-neutral-900 border-l border-neutral-700 p-4 transition-colors disabled:cursor-not-allowed"
          :disabled="
            (pageLastSession || 0) >= (data?.sessions.total || 0) ? true : false
          "
        >
          Next page
        </button>
      </div>
    </div>
    <div class="flex flex-col divide-y divide-neutral-700">
      <div
        v-for="session in data?.sessions.data"
        class="flex h-full flex-col gap-2 p-4 align-middle"
        :class="session.is_using ? 'bg-neutral-900' : 'bg-neutral-950'"
      >
        <div class="flex items-center justify-between gap-2">
          <div
            class="flex items-center gap-2"
            :class="session.is_using ? 'text-brand-50 font-bold' : ''"
          >
            <Icon name="pixelarticons:device-laptop" :size="18" />
            <span>
              {{ session.ip }}
            </span>
          </div>
          <span class="monospace-body text-default-font/60">
            #{{ session.id }}
          </span>
        </div>
        <p class="monospace-body text-default-font/60">
          {{ session.user_agent }}
        </p>
        <p v-if="session.is_using" class="text-brand-50">
          Your current session
        </p>
        <p v-else>
          Last used <NuxtTime :datetime="session.last_used" relative />
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const data = ref<UserSessions>()
const page = ref(1)
const perPage = ref(5)
const pageFirstSession = computed(() => perPage.value * (page.value - 1) + 1)
const pageLastSession = computed(() =>
  perPage.value * page.value >= (data.value?.sessions.total || 0)
    ? data.value?.sessions.total
    : perPage.value * page.value
)

data.value = await $fetch(
  `/api/user/sessions?page=${page.value}&per_page=${perPage.value}`,
  {
    method: 'GET',
  }
)
</script>
