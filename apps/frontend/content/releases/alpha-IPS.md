---
version: alpha-IPS
released: 2023-11-12T16:14:18Z
num: 4
---

#### Changes

- Templates are no longer stored locally, instead, they are fetched from an external repository that [you can find here](https://github.com/BlueprintFramework/framework). This not only allows us to offer community-made templates, it also significantly reduces the size of Blueprint releases.
- Switch out `README.md` placeholder files with `.gitkeep` files.
- Change error for unknown `-init` template value in `blueprint.sh`.
- When upgrading, `blueprint.sh` now no longer wipes the `tools/tmp` folder as it's expected to be empty.
- Improve formatting in `README.md`, add a new "Related Links" and "Introduction" section and add line breaks between sections.
- Custom installation scripts may now check if their extension is installed through a developer command with `$BLUEPRINT_DEVELOPER` and read the version of Blueprint with `$BLUEPRINT_VERSION`.
- Add new `^#installmode#^` [placeholder](/docs/concepts/placeholders) for detecting if your extension is built through developer commands.
- Add new `^#blueprintversion#^` [placeholder](/docs/concepts/placeholders), which, as the name suggests, gets replaced with the host's Blueprint version.
- Address multiple ShellCheck advisories for `blueprint.sh`. ([SC2129](https://www.shellcheck.net/wiki/SC2129), [SC2162](https://www.shellcheck.net/wiki/SC2162), [SC2236](https://www.shellcheck.net/wiki/SC2236))
- `h`, `-h` and `--h` will now trigger the help menu as well.
- Consider `awk`, `grep` and `git` dependencies and check for them in `blueprint.sh`.
- Remove Ko-Fi from `.github/FUNDING.yml`. We no longer accept Ko-Fi donations, donate on Liberapay instead.
- Update Node.JS installation instructions on `README.md` as the automatic installation script has been deprecated. ([#12](https://github.com/BlueprintFramework/framework/pull/12))
- Add new "expose" option for extension exporting. This option will host your extension on a self-hosted URL for two minutes to make downloading the exported extension to your device easier.
- Placeholder logs that are displayed when running `-install` are now grouped.
- `blueprint.sh` now keeps track of developer mode through `lib-db` instead of using a separate file.
- Add new `bp:sync` Artisan command to sync multiple database values with `blueprint.sh`.
- Add new `bp:latest` Artisan command that prints the latest Blueprint version's name.
- New `lib-grabenv` library for "grabbing" values from the environment variables file in the Pterodactyl folder.
- Use environment variables for parsing Pterodactyl folder locations and the current Blueprint version to it's libraries.
- Add new `-info` command that displays some information about your panel in a neofetch-like way.
- Blueprint's admin page can now show if your installation uses an outdated version of the framework and has a new button next to the website button.
- Make minor changes to the CSS of [$blueprint](/docs/lib/methods)'s admin notifications to make them fit in better with the admin panel.
- Some extension folders (private, assets, public) have moved to new locations.
- The website icon on extension admin pages now changes depending on what URL it's linked to.
- Add new SVG and cutout versions of the Blueprint logo in `.blueprint/assets`.

#### Bugfixes

- Some colors were displayed incorrectly in `-help`.
- When ending argument one of `-install` with ".blueprint" it would result in an unrelated error. I'm not sane enough to do text manipulation in bash for some of the installation variables, so now it throws a fatal error telling you what to do instead.
- Extensions previously could not contain spaces in any filename without facing errors regarding placeholders, this is now fixed.
- Running `-wipe` was possible without having developer mode enabled.
- Extension website URLs without a protocol would be renamed to `http://` and would not include the website URL.
- When upgrading Blueprint, developer mode and telemetry id would not be synced until you visited the Blueprint configuration admin page.
- Extension buttons on the extensions page didn't have rounded corners on all sides.

#### Breaking changes

- Node.JS version 17 is no longer supported by Blueprint and will result in a "Unsupported NodeJS version" error.
- Blueprint now uses a partially-changed folder structure for extensions. Extensions that do not use placeholders to find their data directory will break.
