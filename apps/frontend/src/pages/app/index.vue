<template>
  <h1>Hi Emma!</h1>

  <div
    class="grid grid-cols-4 divide-x divide-neutral-700 rounded-3xl border border-neutral-700"
  >
    <div class="space-y-2 p-4">
      <div class="flex items-center gap-1.5">
        <Icon name="memory:account-box" :size="28" mode="svg" />
        <span class="text-xl font-bold">Account</span>
      </div>
      <span class="opacity-75">
        Display name, password, email address, connections, 2-factor
        authentication
      </span>
    </div>
    <div class="space-y-2 p-4">
      <div class="flex items-center gap-1.5">
        <Icon name="memory:cube" :size="28" mode="svg" />
        <span class="text-xl font-bold">Extensions</span>
      </div>
      <span class="opacity-75">
        Create product, configure products, unlist products
      </span>
    </div>
    <div class="space-y-2 p-4">
      <div class="flex items-center gap-1.5">
        <Icon name="memory:chart-bar" :size="28" mode="svg" />
        <span class="text-xl font-bold">Statistics</span>
      </div>
      <span class="opacity-75">
        View statistics, customize graphs, embed widgets
      </span>
    </div>
    <div class="bg-stripes" />
  </div>
</template>
