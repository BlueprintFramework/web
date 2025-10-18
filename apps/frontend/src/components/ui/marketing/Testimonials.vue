<template>
  <div :class="props?.class" class="rounded-3xl border border-neutral-700 p-4">
    <div class="marquee space-y-4">
      <NuxtMarquee
        v-for="(testimonials, index) in allTestimonials"
        :autoFill="true"
        :speed="25"
        :direction="index % 2 == 0 ? 'left' : 'right'"
      >
        <div
          v-for="(testimonial, index) in testimonials"
          :key="`item-${index}`"
          class="w-100 mx-2 h-40 overflow-hidden rounded-3xl border border-neutral-700"
        >
          <div class="flex h-full w-full flex-col divide-y divide-neutral-700">
            <div class="flex items-center justify-between">
              <div class="truncate p-2">
                <h2 class="m-0 truncate">
                  {{ testimonial.name }}
                </h2>
              </div>
              <div
                class="flex h-full items-center gap-1 border-s border-neutral-700 p-2"
              >
                <template v-for="n in 5" :key="n" class="star">
                  <template v-if="n <= testimonial.rating">
                    <Icon
                      name="memory:checkbox-intermediate"
                      class="text-brand-50"
                      :size="20"
                    />
                  </template>
                  <template v-else>
                    <Icon name="memory:checkbox-blank" :size="20" />
                  </template>
                </template>
              </div>
            </div>
            <div class="p-2">
              <p class="text-default-font m-0 p-0">
                "<span class="italic">{{ testimonial.content }}</span
                >"
              </p>
            </div>
          </div>
          <div class="h-0 border-t-0">
            <div
              class="bg-linear-to-b z-5 -top-15 h-15 relative from-transparent to-neutral-950"
            />
          </div>
        </div>
      </NuxtMarquee>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  class: {
    type: String,
    required: false,
  },
})

const allTestimonials = [
  [
    {
      name: 'Futuraura',
      content:
        'Blueprint is honestly one of the best, if not the best. (...) Then Blueprint arrived and completely changed the game.',
      rating: 5,
    },
    {
      name: 'TheBestPenguin',
      content:
        "I've been using Blueprint almost ever since it came out and it is amazing to work on as a developer. I've coded most of my projects on Blueprint after it's release and it's been fantastic.",
      rating: 5,
    },
    {
      name: 'Bradley',
      content:
        'Blueprint expanded Pterodactyl Extensions development in a way the core project couldn’t - It’s made managing, developing, and using multiple extensions at once way easier.',
      rating: 5,
    },
    {
      name: 'Jasper',
      content:
        'Blueprint is an amazing piece of software that allows you to easily upgrade your Pterodactly installations. I would 100% recommend this to anyone who is setting up (or currently running) Pterodactyl.',
      rating: 5,
    },
    {
      name: 'Patrick',
      content:
        'Blueprint is one of the most easy and intuitive frameworks that build quality and extensive features on top of Pterodactyl. Super easy for both end users and extension developers.',
      rating: 5,
    },
    {
      name: 'Michael',
      content:
        "Blueprint is an awesome utility that makes pterodactyl super customizable without the headache of manually installing extension. Don't know how to code, no problem, it's super simple.",
      rating: 5,
    },
    {
      name: 'Hamza',
      content:
        'Blueprint has been an essential utility for me. (...) Its clarity and usability make Blueprint a must-have tool for both private and commercial use.',
      rating: 5,
    },
    {
      name: 'Loapu',
      content:
        'Blueprint gives you the flexibility Pterodactyl needs. You can mod your installation reliably and to an extent not possible with Pterodactyl out of the box or other modifications.',
      rating: 4,
    },
    {
      name: 'Auri',
      content:
        'Blueprint is a great modding platform & it makes it easy to install & manage extensions',
      rating: 5,
    },
  ],
  [
    {
      name: 'ItzArefin',
      content:
        "Blueprint completely changed how I manage my Pterodactyl panel. (...) It has saved me countless hours and made my entire setup so much more stable. I honestly can't imagine using Pterodactyl without it.",
      rating: 5,
    },
    {
      name: 'amogusreal69',
      content:
        "Before, you had to modify files from Pterodactyl itself, but with Blueprint, all I need is run a command, and that's all.",
      rating: 5,
    },
    {
      name: 'WEEZOOKA',
      content:
        'Blueprint is great, it allows you to easily expand the functionality and styling of your pterodactyl install and has a chill community in the discord.',
      rating: 5,
    },
    {
      name: 'Lara',
      content:
        'Blueprint is flexible and up and coming, a decent stop gap measure for installing extensions with better compatibility than file patches',
      rating: 4,
    },
    {
      name: 'Venco',
      content:
        "Blueprint is great for flexibility while keeping stability. It's easy to install Blueprint and its extensions and is such a time saver!",
      rating: 4,
    },
    {
      name: 'Aditya',
      content:
        "It is by far the best thing to happen in whole Pterodactyl community, I've been using blueprint for a quite a time now and it has given me the best experience by far <3",
      rating: 5,
    },
    {
      name: 'Zephrynis',
      content:
        'Blueprint has completely overhauled the way in which I use Pterodactyl and how I will use it for the foreseeable future. (...) Blueprint FTW!!!',
      rating: 5,
    },
    {
      name: 'Janosch',
      content:
        'It’s just nice not having to worry about extensions conflicting or breaking each other, while being able to add add-ons without thinking whether it’ll break the Pterodactyl installation. I love it. Really.',
      rating: 5,
    },
    {
      name: 'Willow',
      content:
        'I love blueprint because of the professional experience I am able to provide for my friends to host their own servers!',
      rating: 5,
    },
  ],
]
</script>
