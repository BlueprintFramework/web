<template>
  <div
    class="grid grid-cols-2 divide-x divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700"
  >
    <div class="min-h-55 flex flex-col divide-y divide-neutral-700">
      <div class="space-y-2 p-4">
        <h2>Devices</h2>
        <p>
          This is a list of devices that are currently signed in to your
          Blueprint account. You can deauthorize devices by clicking them.
        </p>
      </div>
      <div class="bg-stripes h-full" />
      <client-only>
        <div class="flex items-center justify-between">
          <button
            class="disabled:text-default-font/40 hover:not-disabled:text-brand-50 hover:not-disabled:bg-neutral-900 cursor-pointer border-r border-neutral-700 p-4 transition-colors disabled:cursor-not-allowed"
            :disabled="page > 1 ? false : true"
            @click="page--"
          >
            <Icon name="memory:chevron-left" :size="24" mode="svg" />
          </button>
          <div class="p-4">
            <span>
              Sessions {{ pageFirstSession }}-{{ pageLastSession }}
              of
              {{ data?.sessions.total }}
            </span>
          </div>
          <button
            class="disabled:text-default-font/40 hover:not-disabled:text-brand-50 hover:not-disabled:bg-neutral-900 cursor-pointer border-l border-neutral-700 p-4 transition-colors disabled:cursor-not-allowed"
            :disabled="
              (pageLastSession || 0) >= (data?.sessions.total || 0)
                ? true
                : false
            "
            @click="page++"
          >
            <Icon name="memory:chevron-right" :size="24" mode="svg" />
          </button>
        </div>
      </client-only>
    </div>
    <client-only>
      <div class="grid" :class="`grid-rows-${perPage}`">
        <div
          v-for="session in data?.sessions.data"
          class="flex flex-col justify-center gap-2 overflow-hidden border-neutral-700 p-4 align-middle transition-colors"
          :class="
            (session.is_using
              ? 'bg-neutral-900 '
              : 'group cursor-pointer bg-neutral-950 hover:bg-red-950 ') +
            (session != data?.sessions.data.at(-1) ? 'border-b' : 'border-b-0')
          "
        >
          <div class="flex items-center justify-between gap-2">
            <div
              class="flex items-center gap-2 transition-colors"
              :class="
                session.is_using
                  ? 'text-brand-50 font-bold'
                  : 'group-hover:text-red-400'
              "
            >
              <Icon name="pixelarticons:device-laptop" :size="18" />
              <span>
                {{ session.ip }}
              </span>
            </div>
            <div
              v-if="!session.is_using"
              class="flex items-center gap-1 font-bold text-transparent transition-colors group-hover:text-red-400"
            >
              <Icon name="pixelarticons:minus" :size="18" />
              <span>Deauthorize</span>
            </div>
          </div>
          <p
            class="monospace-body text-default-font/60 truncate transition-colors"
            :class="session.is_using ? '' : 'group-hover:text-red-400/60'"
          >
            {{ session.user_agent }}
          </p>
          <p v-if="session.is_using" class="text-brand-50">
            Your current session
          </p>
          <p class="transition-colors group-hover:text-red-400" v-else>
            Last used <NuxtTime :datetime="session.last_used" relative />
          </p>
        </div>
        <div
          v-if="!data?.sessions.data[perPage - 1]"
          class="flex flex-col items-center justify-center gap-1 border-t border-neutral-700 p-4"
        >
          <Icon name="pixelarticons:devices" :size="32" />
          <p>New devices will show up here..</p>
        </div>
      </div>
    </client-only>
  </div>
</template>

<script setup lang="ts">
const data = ref<UserSessions>()
const page = ref(1)
const perPage = ref(4)
const loading = ref(false)

const pageFirstSession = computed(() => perPage.value * (page.value - 1) + 1)
const pageLastSession = computed(() =>
  perPage.value * page.value >= (data.value?.sessions.total || 0)
    ? data.value?.sessions.total
    : perPage.value * page.value
)

const fetchSessions = async () => {
  loading.value = true
  try {
    data.value = await $fetch(
      `/api/user/sessions?page=${page.value}&per_page=${perPage.value}`,
      {
        method: 'GET',
      }
    )
  } catch (error) {
    //TODO: properly handle error in the ui as well
    console.error('failed to fetch sessions:', error)
  } finally {
    loading.value = false

    if (
      (data.value?.sessions.page || 0 > 1) &&
      data.value?.sessions.total == 0
    ) {
      page.value--
    }
  }
}

watch(page, () => {
  fetchSessions()
})

await fetchSessions()
</script>
