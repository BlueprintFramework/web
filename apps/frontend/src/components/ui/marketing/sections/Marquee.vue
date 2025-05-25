<template>
  <div class="marquee-container my-12">
    <NuxtMarquee :autoFill="true" :pauseOnHover="true">
      <div
        v-for="(item, index) in items"
        :key="`item-${index}`"
        class="mx-4 inline h-[128px] w-[256px]"
      >
        <NuxtImg
          :src="item.src"
          :height="128"
          :width="256"
          class="rounded-2xl"
        />
      </div>
    </NuxtMarquee>
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
