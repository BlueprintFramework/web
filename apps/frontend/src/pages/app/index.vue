<template>
  <h1>Hi Emma!</h1>

  <div
    class="grid grid-cols-4 divide-x divide-neutral-700 rounded-3xl border border-neutral-700"
  >
    <div class="p-4">
      <div class="flex items-center gap-1.5">
        <Icon name="memory:account-box" :size="28" mode="svg" />
        <span class="text-xl font-bold">Account settings</span>
      </div>
    </div>
    <div class="p-4">
      <div class="flex items-center gap-1.5">
        <Icon name="memory:cube" :size="28" mode="svg" />
        <span class="text-xl font-bold">Extensions</span>
      </div>
    </div>
  </div>
</template>
