<template>
  <div class="bg-neutral-950">
    <div class="">
      <div
        class="max-md:bg-linear-to-r border-b border-neutral-700 from-red-950 to-neutral-950"
      >
        <div class="container flex">
          <div class="flex flex-col border-neutral-700 md:flex-row md:border-x">
            <div
              class="flex-2/3 md:bg-linear-to-br flex gap-2 border-neutral-700 from-red-950 to-neutral-950 py-2 md:border-r md:px-4 md:py-4"
            >
              <Icon
                name="pixelarticons:warning-box"
                :size="22"
                mode="svg"
                class="mt-0.5 shrink-0 text-red-400"
              />
              <p class="text-default-font/70">
                <span class="font-bold text-red-400">
                  Pterodactyl has ceased operations with Blueprint
                </span>
                following community backlash over actions by Pterodactyl's
                parent company. We are disappointed that this is the course of
                action their team chose to persue. Updates to our Pterodactyl
                modding platform will therefore be limited to security releases
                solely compatible with Pterodactyl v1.
                <br /><br />
                Blueprint as a nonprofit organization, however, is far from
                over. We're going to continue building awesome open-source tools
                for homelabbers and hosting providers:
                <b
                  >One of which is
                  <NuxtLink
                    class="text-link"
                    to="https://hydrodactyl.dev"
                    target="_blank"
                    >Hydrodactyl</NuxtLink
                  >, the Pterodactyl-fork built and funded by Blueprint.</b
                >
                We're also launching some brand new projects in the coming
                months, so stay tuned! <br /><br />
                Join the conversation in our
                <NuxtLink
                  class="text-link"
                  target="_blank"
                  to="https://discord.com/servers/blueprint-1063548024825057451"
                  >Discord community</NuxtLink
                >
                and follow us on
                <NuxtLink
                  class="text-link"
                  target="_blank"
                  to="https://bsky.app/profile/blueprint.zip"
                  >Bluesky</NuxtLink
                >
                for the latest updates.
              </p>
            </div>
            <div
              class="md:flex-1/3 md:bg-linear-to-br space-y-2 bg-red-400 py-2 max-md:my-4 max-md:rounded-3xl max-md:p-3 md:px-4 md:py-4"
            >
              <h2>Support independent open-source development</h2>
              <p>
                Blueprint is a nonprofit organization that builds free &
                open-source software used by many homelabbers and hosting
                providers.
              </p>
              <p class="font-bold">
                If our software has provided you at least $5 of value, please
                consider donating.
              </p>
              <a
                href="https://hcb.hackclub.com/donations/start/blueprint/tiers/custom"
                target="_blank"
                tabindex="-1"
              >
                <button
                  class="flex cursor-pointer items-center gap-1 rounded-full bg-white px-3 py-1.5 font-bold text-neutral-950 transition-colors hover:bg-white/80"
                >
                  <span> Donate to our nonprofit organization directly </span>
                  <Icon name="memory:chevron-right" :size="22" />
                </button>
              </a>
              <p class="text-xs italic">
                (we're also on
                <a
                  href="https://github.com/sponsors/blueprintframework"
                  class="text-link"
                  target="_blank"
                  >GitHub sponsors</a
                >)
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
