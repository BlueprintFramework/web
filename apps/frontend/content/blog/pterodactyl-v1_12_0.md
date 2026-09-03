---
title: Pterodactyl v1.12.0 is now compatible
description: We've released a new Blueprint update that makes us compatible with the latest version of Pterodactyl
author: Emma
thumbnail: pterodactyl-v1_12-compatibility.jpg
date: 1/16/2026 18:00 (CEDST)
num: 2
---

Pterodactyl recently released `v1.12.0`, which Blueprint was not compatible with.
I'm happy to share that we've figured out a way to make the latest version compatible, without breaking existing extensions.

### The issues we ran into

We ran into a few issues trying to port Blueprint to the latest version of Pterodactyl.
These have now been taken care of, but it caused a few delays in the process of making this release.

- I (Emma) was on break while the Pterodactyl update got announced and released. The break lasted until January 12th, while `v1.12.0` released January 6th.
- Certain dependency updates caused Blueprint themes to break. We've been able to take care of this issue by keeping specific dependencies on their old versions.
- A newer version of Webpack stopped providing polyfills for functions like `path`. This was taken care of by Pterodactyl, but we had to add an alias for extensions still using the old library.

### What changed?

> Check out the [more detailed changelog](/releases/beta-2026-01) for more information about what changed this release.

We've made Blueprint officially compatible with `v1.12.0`. The latest update may still work on `v1.11.11`, but that version is no longer officially supported and we recommend [updating your panel](/guides/admin/updatepanel).

Besides making compatibility happen, we've added an onboarding modal visible for whenever you install Blueprint for the first time (or update to any version with the modal for the first time)!

### Final words

[Consider donating to Blueprint!](https://hcb.hackclub.com/donations/start/blueprint)
Donations help us set a solid foundation for the future of Blueprint, and solidify it's place as the industry-leading platform for Pterodactyl panel modding.
