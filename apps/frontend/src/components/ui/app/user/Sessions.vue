<template>
  <div
    class="grid grid-cols-1 divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 md:grid-cols-2 md:divide-x"
  >
    <div
      class="md:min-h-55 flex flex-col divide-y divide-neutral-700 border-b border-neutral-700 md:border-b-0"
    >
      <div class="space-y-2 p-4">
        <h2>Devices</h2>
        <p>
          This is a list of devices that are currently signed in to your
          Blueprint account. You can deauthorize devices by clicking them.
        </p>
      </div>
      <div class="bg-stripes hidden h-full md:block" />
      <client-only>
        <div class="flex items-center justify-between">
          <button
            class="disabled:text-default-font/40 hover:not-disabled:text-brand-50 hover:not-disabled:bg-neutral-900 cursor-pointer border-r border-neutral-700 p-4 transition-colors disabled:cursor-not-allowed"
            :disabled="page > 1 ? false : true"
            @click="page--"
          >
            <Icon name="memory:chevron-left" :size="24" mode="svg" />
          </button>
          <div class="overflow-auto p-4">
            <span class="text-nowrap">
              Sessions {{ pageFirstSession }}-{{ pageLastSession }}
              of
              {{ data?.sessions?.total }}
            </span>
          </div>
          <button
            class="disabled:text-default-font/40 hover:not-disabled:text-brand-50 hover:not-disabled:bg-neutral-900 cursor-pointer border-l border-neutral-700 p-4 transition-colors disabled:cursor-not-allowed"
            :disabled="
              (pageLastSession || 0) >= (data?.sessions?.total || 0)
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
      <div
        class="grid"
        :class="`grid-rows-${sessionsCount == perPage ? perPage : sessionsCount + 1}`"
      >
        <div
          v-for="session in data?.sessions?.data"
          class="flex flex-col justify-center gap-2.5 overflow-hidden border-neutral-700 p-4 align-middle transition-colors"
          :class="
            (session.is_using
              ? 'bg-neutral-900 '
              : 'group cursor-pointer bg-neutral-950 ') +
            (session != data?.sessions?.data.at(-1) ? 'border-b' : 'border-b-0')
          "
          @click="!session.is_using && deleteSession(session.id)"
        >
          <div class="flex items-center justify-between gap-2">
            <div
              class="flex items-center gap-2"
              :class="session.is_using ? 'text-brand-50 font-bold' : ''"
            >
              <Icon
                :name="determineDeviceIcon(session.user_agent)"
                :size="18"
              />
              <span>
                {{ session.ip }}
              </span>
            </div>
            <ElementsButtonSmall
              v-if="!session.is_using"
              label="Forget device"
              class="group-hover:text-brand-50 group-hover:bg-neutral-800"
            />
          </div>
          <p
            class="monospace-body text-default-font/60 truncate transition-colors"
          >
            {{ session.user_agent }}
          </p>
          <p v-if="session.is_using" class="text-brand-50">
            Your current session
          </p>
          <p v-else>
            Last used <NuxtTime :datetime="session.last_used" relative />
          </p>
        </div>
        <div
          v-if="!data?.sessions?.data[perPage - 1]"
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
const data = ref<{ sessions?: UserSessions; errors?: ApiError }>()
const page = ref(1)
const perPage = ref(4)
const loading = ref(false)
const deleting = ref(false)

const pageFirstSession = computed(() => perPage.value * (page.value - 1) + 1)
const pageLastSession = computed(() =>
  perPage.value * page.value >= (data.value?.sessions?.total || 0)
    ? data.value?.sessions?.total
    : perPage.value * page.value
)
const sessionsCount = computed(() => data.value?.sessions?.data.length ?? 0)

const determineDeviceIcon = (ua: string) => {
  if (/android/i.test(ua)) return 'pixelarticons:android'
  if (/mobile|iphone|ipad|ipod/i.test(ua)) return 'pixelarticons:device-phone'
  return 'pixelarticons:device-laptop'
}

const fetchSessions = async () => {
  if (loading.value) return

  loading.value = true
  try {
    data.value = await $fetch(
      `/api/user/sessions?page=${page.value}&per_page=${perPage.value}`,
      {
        method: 'GET',
      }
    )

    if (!data.value?.sessions) {
      throw data.value?.errors
    }
  } catch (error) {
    console.error('failed to fetch sessions:', error)
  } finally {
    loading.value = false

    if (
      (data.value?.sessions?.page || 0 > 1) &&
      data.value?.sessions?.total == 0
    ) {
      page.value--
    }
  }
}

const deleteSession = async (sessionId: number) => {
  if (deleting.value || loading.value) return

  deleting.value = true
  try {
    await $fetch(`/api/user/sessions/${sessionId}`, {
      method: 'DELETE',
    })

    await fetchSessions()
  } catch (error) {
    console.error('failed to delete session:', error)
  } finally {
    deleting.value = false
  }
}

watch(page, () => {
  fetchSessions()
})

await fetchSessions()
</script>
