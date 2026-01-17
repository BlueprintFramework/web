<template>
  <div
    class="upsell-container group cursor-pointer border-neutral-700 bg-neutral-900 md:border-b"
    @click="link?.click()"
  >
    <div
      class="bg-linear-to-r min-h-25 flex w-full flex-col gap-2 from-neutral-900 via-neutral-900 to-neutral-950/50 md:p-8"
    >
      <h2>The game-hosting industry builds on Blueprint</h2>
      <p>
        Pterodactyl's favorite modding community. Develop, collaborate and
        install extensions with the extension platform that puts you first.
      </p>
      <a href="/" tabindex="-1" ref="link">
        <button
          class="text-brand-50 flex cursor-pointer items-center gap-1 font-bold transition-all group-hover:gap-2"
        >
          <span> Learn more </span>
          <Icon name="memory:chevron-right" :size="22" />
        </button>
      </a>
    </div>
  </div>
</template>

<style scoped>
.upsell-container {
  background: url(/img/infoblur_alt.jpeg) no-repeat;
  background-size: 50%;
  background-position: center right;
}
</style>

<script setup lang="ts">
const link = useTemplateRef('link')
</script>
