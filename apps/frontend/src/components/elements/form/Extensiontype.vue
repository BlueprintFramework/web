<template>
  <div>
    <p class="mb-2 text-sm font-medium text-gray-300">Extension Type</p>
    <div class="grid grid-cols-2 divide-x divide-blue-800">
      <div>
        <ElementsButton
          class="w-full rounded-e-none border-e-0"
          :active="type == 'extension'"
          @click="type = 'extension'"
        >
          <div class="flex flex-col items-center">
            <div class="flex items-center gap-2">
              <Icon name="memory:cube" />
              <span>Extension</span>
            </div>
          </div>
        </ElementsButton>
      </div>
      <div>
        <ElementsButton
          class="w-full rounded-s-none border-s-0"
          :active="type == 'theme'"
          @click="type = 'theme'"
        >
          <div class="flex flex-col items-center">
            <div class="flex items-center gap-2">
              <Icon name="memory:image" />
              <span>Theme</span>
            </div>
          </div>
        </ElementsButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const type = defineModel<ExtensionType>()
</script>
