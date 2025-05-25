<template>
  <div class="marquee-container absolute left-0 w-full">
    <div class="relative flex overflow-x-hidden">
      <div
        class="animate-marquee flex items-center justify-between whitespace-nowrap py-12"
      >
        <div
          v-for="(item, index) in items"
          :key="`item-${index}`"
          class="mx-4 inline"
        >
          <UiMarketingSectionsMarqueeItem
            :name="item.name"
            :src="item.src"
            :href="item.href"
          />
        </div>
      </div>

      <div
        class="animate-marquee2 absolute top-0 flex items-center justify-between whitespace-nowrap py-12"
      >
        <div
          v-for="(item, index) in items"
          :key="`item-${index}`"
          class="mx-4 inline"
        >
          <UiMarketingSectionsMarqueeItem
            :name="item.name"
            :src="item.src"
            :href="item.href"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const items = ref([
  {
    name: 'Custom Server Sort',
    src: '/img/extensions/customserversort.jpg',
    href: 'https://blueprint.zip/browse/customserversort',
  },
  {
    name: 'File Search',
    src: '/img/extensions/filesearch.jpg',
    href: 'https://blueprint.zip/browse/filesearch',
  },
  {
    name: 'Loader',
    src: '/img/extensions/loader.jpg',
    href: 'https://blueprint.zip/browse/loader',
  },
  {
    name: 'Nebula',
    src: '/img/extensions/nebula.jpg',
    href: 'https://blueprint.zip/browse/nebula',
  },
  {
    name: 'Server Importer',
    src: '/img/extensions/serverimporter.jpg',
    href: 'https://blueprint.zip/browse/serverimporter',
  },
  {
    name: 'Server Splitter',
    src: '/img/extensions/serversplitter.jpg',
    href: 'https://blueprint.zip/browse/serversplitter',
  },
  {
    name: 'Social Login',
    src: '/img/extensions/sociallogin.jpg',
    href: 'https://blueprint.zip/browse/sociallogin',
  },
  {
    name: 'Subdomains',
    src: '/img/extensions/subdomains.jpg',
    href: 'https://blueprint.zip/browse/subdomains',
  },
])
</script>

<style scoped>
.marquee-container {
  mask: linear-gradient(
    to right,
    transparent 0%,
    black 20%,
    black 80%,
    transparent 100%
  );
  -webkit-mask: linear-gradient(
    to right,
    transparent 0%,
    black 20%,
    black 80%,
    transparent 100%
  );
}
</style>
