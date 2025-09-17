<template>
  <div
    class="grid grid-cols-1 divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 md:grid-cols-2 md:divide-x"
  >
    <div class="divide-y divide-neutral-700">
      <div class="space-y-2 p-4">
        <h2>Email address</h2>
        <p>View and update the email address associated with your account.</p>
      </div>
      <div class="bg-stripes hidden h-full md:block" />
    </div>
    <div class="divide-y divide-neutral-700">
      <div class="p-4">
        <ElementsButton @click="isModalOpen = true" label="Open modal" />

        <ElementsModal
          :is-open="isModalOpen"
          title="My Modal"
          @close="isModalOpen = false"
        >
          Your content here

          <template #footer>
            <ElementsButton label="wow" />
            <ElementsButton label="wow" :disabled="true" />
            <ElementsButton label="hi" color="danger" />
            <ElementsButton label="hi" color="danger" :disabled="true" />
          </template>
        </ElementsModal>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const isModalOpen = ref(false)
</script>
