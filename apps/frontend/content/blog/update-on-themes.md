---
title: The state of themes on Blueprint
description: An update on our recent efforts to make theming easier on our platform
author: Emma
thumbnail: themes-on-blueprint.jpg
date: 12/8/2025 20:00 (CEDST)
num: 1
---

Ever since the first functional version of Blueprint, themes have been complicated to build and maintain. I recently introduced a new, easier way to build consistent themes through CSS variables and shipped it with [beta-2025-11](/releases/beta-2025-11).

It broke, often.

After weeks of experimenting, **I'm making the difficult decision to remove the "improved" theming functionality in beta-2025-12**.

### The need for consistent theming support

Creating a consistent theme in Blueprint has been difficult, unmaintainable and confusing to say the least. Themes such as [Nebula](/browse/nebula) modify hundreds of seemingly-random class names and deal with complicated CSS selectors to adjust styling.

> I've written an [unofficial guide about making Blueprint themes](https://prpl.blog/#read/custom-pterodactyl-theme) over on my personal blog. I think it can give you enough of an idea of the wizard knowledge you need to make proper themes in Blueprint. I was quite optimistic about it back then, huh.

Despite this, the community has [built](/browse/darkenate) [some](/browse/euphoriatheme) [great](/browse/catppuccindactyl) [themes](/browse/recolor) with Blueprint.

### The theoretical solution

Pterodactyl's user-side frontend is built with React, [Tailwind](https://tailwindcss.com) and a few other technologies. Tailwind condenses CSS properties to simple CSS classes. To change the background color of an element to white, for example, you would add `bg-white` to the list it's classes. This is similar to [inline styles](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/style), but much shorter and the preferred method for many developers.

Tailwind allows for defining a list of colors, fonts, etc for your website (the "Tailwind configuration"). This helps with keeping your site consistent, while retaining the flexibility. Tailwind is often compared to [Bootstrap](https://getbootstrap.com) (of which an old version is used for the Pterodactyl admin panel), but with more flexibility over Bootstrap's more standard UI.

> I've personally used both Tailwind and Bootstrap on a few websites. The Blueprint site you're on is actually built with Tailwind, while our legacy site was built with Bootstrap.

In theory, allowing extensions to modify the Tailwind configuration would allow themes to change the colors, fonts, spacing and more through a simple configuration. The best way I could think of doing this was through CSS variables.

CSS variables;

- Wouldn't require the panel's frontend assets to be rebuilt when adjusting the color scheme. This would allow them to be adjusted on the fly through, for example, a theme's admin dashboard.
- Could be edited in and outside of the panel's frontend asset bundle.
- Priority would be handled by the browser, and would prevent me from having to build out a complicated parser to handle theme values.

### The first roadblock

Pterodactyl uses a few (outdated) libraries. Lots of extensions and panel components rely on these libraries. Upgrading them to their latest versions would break more than it would fix.

The version of Tailwind used by the Pterodactyl panel (v3.0.24, [package.json](https://github.com/pterodactyl/panel/blob/9b703fb40f2c8d0fe08c15a08d9f919a9cee7fa3/package.json#L47)) was able to use CSS variables for configuration options. It ran into problems, however, when calculating the [opacity](https://v3.tailwindcss.com/docs/text-color#changing-the-opacity) of colors set this way.

> It's worth noting that Tailwind v4's configuration is written in a CSS file. Tailwind v3's configuration **is not**.

Upgrading Tailwind to v3.4.17 seemed to fix this issue, so we went with that and it worked.. sort of.

### The second roadblock

I wrote [a simple theme](https://github.com/prplwtf/tailwindfourblueprint) to test the theming functionality. It seemed to only apply the changes to a few of the panel's elements.

After diving deeper, the issue seemed to lie in Pterodactyl's heavy use of [twin.macro](https://github.com/ben-rogerson/twin.macro). Removing Twin would require editing it out of basically the entire frontend, and existing themes for Blueprint breaking. Besides, a lot of extensions depend on it, so kicking it out isn't really an option.

> Projects like Twin aren't built to be hyper-extensible, especially in the way we're doing it. Our use-case is anything but standard.

At this point, I was pretty much a week into building this feature already, and it seemed to work, so I called it a day.

### Don't deploy on Fridays

On November 28th, I shipped [beta-2025-11](/releases/beta-2025-11).

A portion of users immediately started experiencing errors when building the frontend asset bundle. I shipped a few patches until it (largely) worked.

The final patch solved most issues. A percentage of users, however, started experiencing issues with a few elements' colors turning black (bug was often referred to "black graphs"). The exact reason why it worked on my and a few others' Pterodactyl panels wasn't found, but there's a good chance cache was to blame.

### The third and final roadblock

I spent the next week experimenting even more;

- Using PostCSS together with a script to go through CSS variables and apply them to the Tailwind configuration on build time. Even though it didn't even work properly in the first place, it also removed the ability for these CSS variables to be adjusted on the fly, as they'd be indexed only when building the frontend asset bundle.
- Replacing Twin's `theme()` function. This was moreso a proof of concept. It was abandoned as it only fixed issues for a small amount of the affected components that made use of the function.
- Messing with libraries' configuration options.

None of these seemed to fix the "black graphs" bug, and when they did, I couldn't replicate the fix on other Pterodactyl installations.

> I was generally just pushing buttons until something happened, with a large amount of the "fixes" coded by AI. The fixes listed above never made it to a release. I was pretty desperate to find a solution to have this **just work**, but after weeks of trying, it's time to admit defeat.

### A final decision

This December there will be a release that will roll-back the theming-related changes made in [beta-2025-11](/releases/beta-2025-11). The [related documentation article](/docs/themes/colors) has been unlisted and [beta-2025-11](/releases/beta-2025-11) will be marked as unsupported once beta-2025-12 releases.

Thank you for your patience with me and the rest of the team.

#### Things break sometimes

Blueprint has been in a stable state over the past year. Due to this, it might be easy to forget that Blueprint, after all, is still in it's beta stage.

That said, I should have never released Blueprint in this state. I think it's important for us to do more extensive testing before shipping a release, through CI and test deployments for example.

People are people, and people make mistakes.
