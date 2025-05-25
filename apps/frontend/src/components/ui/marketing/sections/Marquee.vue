<template>
  <div class="marquee my-14">
    <NuxtMarquee :autoFill="true" :pauseOnHover="true" :speed="50">
      <div
        v-for="(item, index) in items"
        :key="`item-${index}`"
        class="mx-4 inline h-[128px] w-[256px]"
      >
        <NuxtLink :to="`/extensions/${item.identifier}`">
          <div class="transition-transform hover:scale-95">
            <div
              class="absolute h-[128px] w-[256px] bg-neutral-950/50 opacity-0 backdrop-blur-none transition-all hover:opacity-100"
            >
              <Icon
                name="memory:arrow-up-right-box"
                :size="48"
                mode="svg"
                class="top-6/12 left-6/12 -translate-3/6 absolute"
              />
            </div>
            <NuxtImg
              :src="item.src"
              :height="128"
              :width="256"
              :alt="item.name"
              class="rounded-2xl"
            />
          </div>
        </NuxtLink>
      </div>
    </NuxtMarquee>
  </div>
</template>

<script setup lang="ts">
const items = ref([
  {
    name: 'Custom Server Sort',
    identifier: 'customserversort',
    src: '/img/extensions/customserversort.jpg',
    href: 'https://blueprint.zip/browse/customserversort',
  },
  {
    name: 'File Search',
    identifier: 'filesearch',
    src: '/img/extensions/filesearch.jpg',
    href: 'https://blueprint.zip/browse/filesearch',
  },
  {
    name: 'Loader',
    identifier: 'loader',
    src: '/img/extensions/loader.jpg',
    href: 'https://blueprint.zip/browse/loader',
  },
  {
    name: 'Nebula',
    identifier: 'nebula',
    src: '/img/extensions/nebula.jpg',
    href: 'https://blueprint.zip/browse/nebula',
  },
  {
    name: 'Server Importer',
    identifier: 'serverimporter',
    src: '/img/extensions/serverimporter.jpg',
    href: 'https://blueprint.zip/browse/serverimporter',
  },
  {
    name: 'Server Splitter',
    identifier: 'serversplitter',
    src: '/img/extensions/serversplitter.jpg',
    href: 'https://blueprint.zip/browse/serversplitter',
  },
  {
    name: 'Social Login',
    identifier: 'sociallogin',
    src: '/img/extensions/sociallogin.jpg',
    href: 'https://blueprint.zip/browse/sociallogin',
  },
  {
    name: 'Subdomains',
    identifier: 'subdomains',
    src: '/img/extensions/subdomains.jpg',
    href: 'https://blueprint.zip/browse/subdomains',
  },
])
</script>

<style scoped>
.marquee {
  mask: linear-gradient(
    to right,
    transparent 0%,
    black 10%,
    black 90%,
    transparent 100%
  );
  -webkit-mask: linear-gradient(
    to right,
    transparent 0%,
    black 10%,
    black 90%,
    transparent 100%
  );
}
</style>
