<template>
  <div
    class="min-h-30 flex flex-col items-center justify-between gap-4 rounded-3xl border border-neutral-700 bg-[url(/img/banners.png)] bg-cover bg-right bg-no-repeat p-4 sm:flex-row"
  >
    <div class="w-full space-y-1 sm:w-auto">
      <p class="text-2xl font-bold">What are we building next?</p>
      <p class="text-default-font/60">
        Give your next extension a head start by publishing it to Blueprint.
      </p>
    </div>
    <ElementsButton
      label="New extension"
      class="w-full sm:w-auto"
      @click="modalOpen.create_new = true"
    />
  </div>

  <ElementsModal
    :is-open="modalOpen.create_new"
    :closable="true"
    title="New extension"
    @close="modalOpen.create_new = false"
  >
    <template #default>
      <div class="grid grid-cols-2 gap-4">
        <ElementsButton
          class="min-h-20 w-full !p-4"
          @click="
            () => {
              modalOpen.create_new = false
              modalOpen.new_extension = true
            }
          "
        >
          <div class="flex w-full flex-col items-center gap-2">
            <Icon name="pixelarticons:plus" :size="28" />
            <p>I'm building a new extension</p>
          </div>
        </ElementsButton>
        <ElementsButton
          class="min-h-20 w-full !p-4"
          @click="
            () => {
              modalOpen.create_new = false
              modalOpen.distributed_extension = true
            }
          "
        >
          <div class="flex w-full flex-col items-center gap-2">
            <Icon name="pixelarticons:link" :size="28" />
            <p>I've already published my extension to distributors</p>
          </div>
        </ElementsButton>
      </div>
    </template>

    <template #footer>
      <ElementsButton
        label="Cancel"
        class="w-full md:w-auto"
        @click="modalOpen.create_new = false"
      />
    </template>
  </ElementsModal>

  <ElementsModal
    :is-open="modalOpen.new_extension"
    :closable="true"
    title="New extension"
    @close="modalOpen.new_extension = false"
  >
    <template #default> </template>

    <template #footer>
      <ElementsButton
        label="Back"
        class="w-full md:w-auto"
        @click="
          () => {
            modalOpen.new_extension = false
            modalOpen.create_new = true
          }
        "
      />
      <ElementsButton
        label="Create"
        class="order-first w-full md:order-[unset] md:w-auto"
      />
    </template>
  </ElementsModal>

  <ElementsModal
    :is-open="modalOpen.distributed_extension"
    :closable="true"
    title="New extension"
    @close="modalOpen.distributed_extension = false"
  >
    <template #default>
      lorem ipsum dolor whatever the fuck comes next
    </template>

    <template #footer>
      <ElementsButton
        label="Back"
        class="w-full md:w-auto"
        @click="
          () => {
            modalOpen.distributed_extension = false
            modalOpen.create_new = true
          }
        "
      />
      <ElementsButton
        label="Create"
        class="order-first w-full md:order-[unset] md:w-auto"
      />
    </template>
  </ElementsModal>
</template>

<script setup lang="ts">
const modalOpen = ref({
  create_new: false,
  new_extension: false,
  distributed_extension: false,
})
</script>
