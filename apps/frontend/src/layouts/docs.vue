<template>
  <div class="flex">
    <div class="flex">
      <div class="w-76">
        <div
          class="w-76 fixed z-10 h-full flex-col space-y-4 overflow-scroll bg-neutral-950"
        >
          <div
            class="bg-linear-to-b sticky top-0 space-y-4 border-b border-neutral-700 from-neutral-950/50 to-transparent p-4 backdrop-blur-sm"
          >
            <BrandWordmark />
            <UiFormInput
              v-model="form.search"
              name="search"
              type="text"
              :rules="[]"
              leading-icon="memory:search"
              placeholder="Search.."
              @validate="void"
            />
          </div>
          <div class="h-1000 space-y-4 px-4">
            <div
              class="divide-y divide-neutral-700 rounded-2xl border border-neutral-700"
            >
              <div class="flex gap-1.5 p-2 align-middle font-bold">
                <Icon
                  name="memory:application-code"
                  :size="22"
                  mode="svg"
                  class="block"
                />
                <span> CLI </span>
              </div>
              <div class="p-2">
                <button
                  class="hover:text-brand-50 w-full text-start transition-colors"
                >
                  <span>Commands</span>
                </button>
                <button
                  class="hover:text-brand-50 w-full text-start transition-colors"
                >
                  <span>agentic ai b2b saas</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div>
        <div
          class="bg-linear-to-b fixed h-full w-[1px] from-neutral-800 via-neutral-500 to-neutral-800"
        />
      </div>
    </div>
    <div class="max-w-200 container space-y-12 px-4 py-12">
      <slot />
    </div>
  </div>

  <div
    class="fixed inset-0 top-0 -z-10 h-[50vh] w-full bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:30px_30px] bg-[position:-5px_-5px]"
  >
    <div class="bg-linear-to-b h-full w-full from-transparent to-neutral-950" />
  </div>
</template>

<script setup lang="ts">
const form = ref({
  search: '',
})
</script>
