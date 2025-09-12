<template>
  <div
    class="flex flex-col items-center rounded-3xl border border-red-800 bg-red-950 text-red-400 md:flex-row"
  >
    <div
      class="flex w-full flex-row items-center gap-2 border-b border-red-800 p-4 md:block md:w-auto md:border-b-0"
    >
      <Icon name="memory:email" :size="24" mode="svg" />
      <span class="h2 md:hidden">Unverified email</span>
    </div>
    <div class="w-full border-red-800 p-4 md:w-auto md:border-s">
      Your account is limited until you
      <NuxtLink to="/auth/verify" class="text-link hover:!text-red-300"
        >verify your email address</NuxtLink
      >.
    </div>
  </div>
</template>
