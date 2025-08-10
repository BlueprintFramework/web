<template>
  <div :class="props?.class" class="marquee space-y-4">
    <NuxtMarquee
      v-for="(testimonials, index) in allTestimonials"
      :autoFill="true"
      :speed="35"
      :direction="index % 2 == 0 ? 'left' : 'right'"
    >
      <div
        v-for="(testimonial, index) in testimonials"
        :key="`item-${index}`"
        class="w-100 h-50 mx-2 overflow-hidden rounded-3xl border border-neutral-700"
      >
        <div class="flex h-full w-full flex-col divide-y divide-neutral-700">
          <div class="p-2">
            <h2 class="m-0 truncate">{{ testimonial.name }}</h2>
          </div>
          <div class="p-2">
            <p class="m-0 p-0">{{ testimonial.content }}</p>
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
        'Blueprint is honestly one of the best, if not the best. Before it came along, adding extensions, themes, or other modifications to the Pterodactyl panel had to be done manually. Then Blueprint arrived and completely changed the game. It cut out all the hassle between buying and actually using extensions. Back then, I constantly ran into issues installing standalone extensions: endless bugs, reaching out to support, and getting lost trying to figure out which file was supposed to be replaced based on some vague readme. Blueprint solved all of that.',
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
        'Blueprint expanded Pterodactyl Extensions development in a way the core project couldn’t - It’s made managing, developing, and using multiple extensions at once way easier. ',
      rating: 5,
    },
    {
      name: 'Jasper',
      content:
        'Blueprint is an amazing piece of software that allows you to easily upgrade your Pterodactly installations. I would 100% recommend this to anyone who is setting up (or currently running) Pterodactyl.',
      rating: 5,
    },
    {
      name: 'Mega004',
      content:
        "I like blueprint because of their support team, they're really understanding even on stressful times (i.e. Pterodactyl Security patch recently).",
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
      name: 'Nucceteere',
      content:
        "Blueprint is a really great framework letting you manage your extensions and themes with ease. Extensions/plugins that doesn't use Blueprint and require you to modify the files yourself usually end up in broken panels but Blueprint tries to mitigate this in a smart way and efficient way.",
      rating: 5,
    },
    {
      name: 'Hamza',
      content:
        'Blueprint has been an essential utility for me. Installing extensions manually always brought up issues, and was very hard to keep up with all the different extensions. Its clarity and usability make Blueprint a must-have tool for both private and commercial use.',
      rating: 5,
    },
    {
      name: 'SuperEvilLuke',
      content:
        'Blueprint is an amazing framework for pterodactyl to easily install and manage extensions for your panel! Works great, with an amazing community!',
      rating: 5,
    },
    {
      name: 'Loapu',
      content:
        'Blueprint gives you the flexibility Pterodactyl needs. You can mod your installation reliably and to an extent not possible with Pterodactyl out of the box or other modifications. In recent updates, the developer made it easier to install and manage, even easier than Pterodactyl itself.',
      rating: 4,
    },
  ],
  [
    {
      name: 'ItzArefin',
      content:
        "Blueprint completely changed how I manage my Pterodactyl panel. Before, I was manually editing core files to get the features I wanted, and every update was a headache. I was always worried I'd break something. Now, with Blueprint, adding extensions and themes is simple and secure. It’s a clean framework that lets you customize your panel without ever touching the core files. It has saved me countless hours and made my entire setup so much more stable. I honestly can't imagine using Pterodactyl without it.",
      rating: 5,
    },
    {
      name: 'Griind',
      content:
        'Being a new player in this whole pterodactyl and self-hosting space, finding Blueprint made the whole "getting-started with customizing pterodactyl\'s panel" so extremely much simpler compared to the dread I got from seeing non-blueprint customizations\' installation documentations. (So long you remember to type the install commands in the correct directory hehe whoops). The community around bp is also extremely friendly and ready to help with whatever incredibly stupid newbie questions I had lol',
      rating: 4,
    },
    {
      name: 'amogusreal69',
      content:
        "I use Blueprint because I hate the current way you install extensions. Before, you had to modify files from Pterodactyl itself, but with Blueprint, all I need is run a command, and that's all.",
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
        "Blueprint is a boon for users who want to customize their Pterodactyl panel but dont have much or no coding skils or knowledge. It is by far the best thing to happen in whole Pterodactyl community, I've been using blueprint for a quite a time now and it has given me the best experience by far <3",
      rating: 5,
    },
    {
      name: 'Zephrynis',
      content:
        'Blueprint has completely overhauled the way in which I use Pterodactyl and how I will use it for the foreseeable future. It has allowed me to extend the feature set to levels I never thought possible without compromising on compatibility with other modifications. Blueprint FTW!!!',
      rating: 5,
    },
    {
      name: 'Janosch',
      content:
        'I like Blueprint because it’s just so easy to use. It’s just nice not having to worry about extensions conflicting or breaking each other, while being able to add add-ons without thinking whether it’ll break the Pterodactyl installation. I love it. Really.',
      rating: 5,
    },
    {
      name: 'Willow',
      content:
        'I love blueprint because of the professional experience I am able to provide for my friends to host their own servers!',
      rating: 5,
    },
    {
      name: 'MkcTao',
      content:
        'Been using Blueprint for a bit over a year, it just works, everytime. You never have to worry about conflicts or updates, it always does exactly what you want. Easy and Reliable.',
      rating: 5,
    },
  ],
]
</script>
