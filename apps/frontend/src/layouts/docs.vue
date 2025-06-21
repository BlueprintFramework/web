<template>
  <div class="flex">
    <div class="flex">
      <div class="w-76">
        <div class="w-76 fixed z-10 h-full bg-neutral-950 p-4">
          <div class="flex-col space-y-4">
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
            <div>
              <!-- docs navigation -->
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
    <div class="max-w-200 container px-4 py-12">
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
