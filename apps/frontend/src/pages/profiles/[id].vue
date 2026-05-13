<template>
  <div class="space-y-12">
    <div class="grid grid-cols-4 gap-4">
      <div
        class="w-full space-y-4 max-xl:col-span-4"
        v-if="publicUser && !pendingUser"
      >
        <ElementsInlinecard
          v-if="publicUser.user.suspended"
          type="danger"
          icon="memory:alert-box-fill"
          class="rounded-3xl"
        >
          This account is suspended. Some content may be unavailable.
        </ElementsInlinecard>
        <div
          class="flex flex-col overflow-hidden rounded-3xl border border-neutral-700"
        >
          <div class="bg-stripes h-15.5 relative border-b border-neutral-700">
            <div
              class="bg-brand-700 h-25 w-25 absolute left-3 top-3 aspect-square rounded-full border border-neutral-700"
            >
              <div class="flex h-full items-center justify-center">
                <span class="text-brand-50 text-5xl font-bold">
                  {{ publicUser.user.name.charAt(0) }}
                </span>
              </div>
            </div>
          </div>
          <div class="h-13">
            <div class="-top-2.75 relative flex flex-row-reverse gap-2 pe-3">
              <NuxtLink
                v-if="publicUser.user.support"
                :to="publicUser.user.support"
                tabindex="-1"
              >
                <ElementsButtonSmall> Support </ElementsButtonSmall>
              </NuxtLink>
              <NuxtLink
                v-if="publicUser.user.id == user?.id"
                to="/app/account"
                tabindex="-1"
              >
                <ElementsButtonSmall> Edit </ElementsButtonSmall>
              </NuxtLink>
            </div>
          </div>
          <div class="p-4 pt-2">
            <p class="text-xl font-bold">
              {{ publicUser.user.name }}
              <span v-if="publicUser.user.pronouns" class="text-sm opacity-50">
                {{ publicUser.user.pronouns }}
              </span>
            </p>
            <div class="flex items-center justify-between">
              <span>
                Member since
                <NuxtTime
                  :datetime="publicUser.user.created"
                  :relative="true"
                />
              </span>
              <span class="text-sm opacity-50">#{{ publicUser.user.id }}</span>
            </div>
          </div>
        </div>
      </div>
      <div
        class="h-[192px] overflow-hidden rounded-3xl border border-neutral-700 max-xl:col-span-4"
        v-else
      >
        <div class="h-full w-full animate-pulse bg-neutral-900" />
      </div>

      <div
        class="col-span-3 grid w-full grid-cols-1 gap-px overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 transition-colors focus-within:border-neutral-500 focus-within:bg-neutral-500 max-xl:col-span-4 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4"
        v-if="publicUser && extensions && !pendingExtensions"
      >
        <ElementsExtensionCard
          v-for="extension in extensions"
          :key="extension.id"
          :extension="extension"
          class="flex flex-col bg-neutral-950"
        />
        <div
          v-if="!(!pendingExtensions && extensions?.length === 0)"
          class="min-h-5 w-[calc(400%+3px)] bg-neutral-950"
        >
          <div class="bg-stripes h-full w-full" />
        </div>
        <div v-else class="col-span-full bg-neutral-950">
          <div
            class="bg-stripes p-9.25 flex h-full flex-col items-center justify-center bg-neutral-950 text-center"
          >
            <p>
              <span
                class="rounded-full border border-neutral-700 bg-neutral-950 px-4 py-3"
              >
                {{ publicUser.user.name }} hasn't published any extensions yet
              </span>
            </p>
          </div>
        </div>
      </div>
      <div
        class="col-span-3 overflow-hidden rounded-3xl border border-neutral-700 max-xl:col-span-4"
        v-else
      >
        <div class="h-full w-full animate-pulse bg-neutral-900" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const { user } = useAuth()

const { data: publicUser, pending: pendingUser } =
  await useAsyncData<PublicUser | null>(
    `publicuser-${route.params.id}`,
    () => $fetch(`/api/users/${route.params.id}`),
    {
      server: false,
      default: () => null,
    }
  )

const { data: extensions, pending: pendingExtensions } = await useAsyncData<
  Extension[]
>(
  `extensions-${route.params.id}`,
  () => $fetch<Extension[]>(`/api/users/${route.params.id}/extensions`),
  {
    server: false,
  }
)

useSeoMeta({
  title: () => publicUser.value?.user.name,
  description: () =>
    `Browse extensions created by ${publicUser.value?.user.name} directly on Blueprint.`,
})
</script>
