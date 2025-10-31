---
version: beta-A428
released: 2024-04-09T20:21:09Z
summary: The infamous bug-riddled beta release
num: 8
---

::card
This release.. was well.. quite rough. Here's a list of changes/bugfixes made for beta-A428-1 and beta-A428-2:

- Extension views were not parsed correctly resulting in empty view pages. This has been fixed in a hotfix.
- Visiting extensions that had custom controllers in your admin panel caused a server error. This has been fixed in a hotfix as well.
- Making use of the new placeholders would result in a bunch of errors, this has been fixed.
- Add a timeout for requests towards the Blueprint API to prevent some pages from loading indefinitely.
- Some elements (like NavigationBar and SubNavigation) now have static element IDs for easier styling.

::

#### Changes

- `blueprint.sh` now has a variable to change the webserver permission user which will persist over updates.
- Having nothing as input in `-init` questions will now select the placeholder option.
- New `adminOnly` scope for custom extension routes, which allows developers to limit pages to administrators.
- Custom dashboard routes can now leave `name` empty to not have them show up inside of the SubNavigation element.
- Developers can now use `SVG`, `GIF`, `WebP`, `PNG` and `JPEG` formats for their extension icons instead of having those formats ignored and forced onto a (possibly corrupted) `icon.jpg` file.
- When building, installing or removing extensions Blueprint will now re-apply webserver permissions.
- Add new extension APIs (`views`, `controllers` and `routers`) which can be enabled from [conf.yml](/docs/configs/confyml). These new extension APIs can be useful for expanding extension functionality even further.
- Each installed extension now gets assigned a filesystem (`blueprint:<identifier>`) automatically which can be used for file uploading, automatic handling of files, etc.
- Extensions can now check if another extension is installed through a new API in [$blueprint](/docs/lib/methods).
- Add new hint to the Blueprint admin page that only shows once and promotes the [`BlueprintFramework/framework`](https://github.com/BlueprintFramework/framework) GitHub repository.
- When using `-upgrade` to update from **this version** to a **later version**, Blueprint will automatically migrate development files over if extension development files are found.
- Improve the codebase of multiple internal services, controllers and views.
- Build files (used for installing/updating extensions) have been made more modular by making use of partials and sections, allowing editing them to be much easier.
- New permissions setting on all installed extensions allowing you to block an extension from impacting certain sections on your panel.
- `-upgrade` will no longer import/update Blueprint's `.git` and `.github` folders.
- `robots.txt` has been changed to allow crawlers on `/extensions/blueprint/index.html`.
- Admin and dashboard wrappers now get imported from a symlinked file dynamically instead of being injected through `blueprint.sh`.
- Docker users are now forced onto the `/app` directory as Pterodactyl directory.
- Change the entire way [placeholders](/docs/concepts/placeholders) work and add some brand new placeholder features.

#### Bugfixes

- Admin CSS from extensions and Blueprint were imported **before** other stylesheets, causing them to override extension CSS.
- The green tag in the admin panel had a bright background color, causing it's text to be hard to read.
- Path towards an extension's database migrations directory was not validated due to a mistyped variable.
- `-debug` was throwing seemingly-random errors when receiving text input, which has now been fixed.
- `-remove` had a leftover string when throwing a specific error, which has now been fixed.
- Blueprint's admin page was making multiple of the same API calls per load, this has now been reduced to one API call.
- Extension admin page was supposed to show an error when the version variable of Blueprint was `::v`, but did not.
- `-upgrade` did not half after encountering an error with fetching remote Blueprint releases.
- Blueprint did not exit after updating it's Pterodactyl installation folder variable.
- Some issues with running Blueprint on Docker have been resolved. While it's still not officially supported, we try to make it work as well as possible.
- Identifiers were able to consist of hyphens (`-`), underscores (`_`), periods (`.`) and spaces. Extensions that did use these characters in their identifiers ended up breaking some parts of the Pterodactyl panel in some way, shape or form.

#### Breaking changes

- Blueprint's GitHub organization has been renamed from `teamblueprint` to `BlueprintFramework`. This should have minimal impact, but some things might break that are related to the GitHub API on older versions.
- `BlueprintVariableService` has been removed. There are no known cases of extensions using this internal library, so it should have minimal effect.
- HTML syntax in extension descriptions will now be escaped for consistency across extension admin pages. There are no known cases of extensions using HTML in their descriptions, so it should have minimal effect.
- `-upgrade` no longer accepts the `dev` argument. Use the new `remote` argument instead.
- The `ignoreAlphabetPlaceholders` flag has been removed. There have been no known cases of this flag being used, so this should have minimal effect.
- [Placeholders](/docs/concepts/placeholders) have changed (for the better) and need to be updated. Extensions with a `alpha` or `indev` target version will still get the placeholders applied like before until the new placeholders are widely adopted. You can force legacy placeholder behavior by using the `forceLegacyPlaceholders` [flag](/docs/concepts/flags).
