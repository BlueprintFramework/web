---
version: alpha-NLM
released: 2024-01-06T22:43:42Z
summary: React extensibility? It's more likely than you may think!
num: 6
---

#### Changes

- Extensions list alongside `misc` folder inside of `.blueprint` has now been removed.
- `parse_yaml` inside of `.blueprint/lib` has been bumped to the latest commit.
- Replace most (if not all) URLs leading to [`ptero.shop`](https://ptero.shop) with [`blueprint.zip`](https://blueprint.zip).
- Update `installWithBlueprintBanner` asset inside of `.blueprint/assets` to include the new URL of Blueprint instead of the old one.
- Change related link description for [`teamblueprint/web`](https://github.com/BlueprintFramework/web) alongside removing the extension list in `README.md`.
- Remove all old versions of the Blueprint logo from `.blueprint/assets`.
- Outdated cache overlay is now a tiny banner at the bottom of the admin panel instead of a fullscreen overlay.
- Add new `logoCoverTextPurple` asset.
- Admin cache no longer directly modifies `pterodactyl.css` but instead goes through a file made specifically for extension css importing.
- Add "Byte" to the Blueprint admin page.
- All file and folder paths defined by an extension are validated upon installing or building it.
- Add a separate function for assinging extention flags to variables for `-install`, `-remove` and `-export`.
- Check for extensions flags through variables instead of checking the `$info_flags` string each if statement that validated a specific flag on `-install`, `-remove` and `-export`.
- Address all [ShellCheck](https://www.shellcheck.net/wiki/) advisories across `blueprint.sh` and it's libraries.
- Add new `developerIgnoreRebuild` flag that allows developers to skip rebuilding all panel assets when editing panel css upon running `-build`.
- Speed up "Checking Dependencies" by dumping `npm ls` and just looking for the module in the `node_modules` folder.
- Add new `-debug` command, which can be used to easily print all recent information and muted logs.
- The `-help` command can now also be triggered by providing zero arguments to the `blueprint` CLI tool.
- Internal placeholders `@version`, `&bp.version&` and `&bp.folder&` have been renamed to `::v` and `::f`.
- Developers can now add components to the Pterodactyl navigation bar, account configuration pages and server management pages through [Components.yml](/docs/configs/componentsyml).
- Add GitHub sponsors section and link to Blueprint's `README.md`.
- Improve error detection across `blueprint.sh`, mostly for extension installation and removal.

#### Bugfixes

- Improve Docker installation check by properly checking for the `.dockerenv` file this time.
- Development files were referred to as "extension files" in `-wipe`.
- "Refresh your cache" overlay appeared to break from time to time.
- `conf.yml` had a different order for "wrapper" and "css" values across "dashboard" and "admin".
- `-info` would show log function names when some values were empty.
- Running `bash blueprint.sh` after installing Blueprint would show a `[WARNING]` log which should be `[FATAL]` for consistency.
- "Checking dependencies" was ran before asking administrators for confirmation when running `-remove`.
- If extension script flags were present but the extension did not have given script, Blueprint would throw a weird error.
- The debug file path defined in `lib/bash_colors.sh` was relative, which would cause an error if not ran in the Pterodactyl directory.
- Extension file paths defined in `conf.yml` could escape the temporarily directory.
- Onboarding admin notification was not being displayed and has now been removed.
- Puzzle navbar button in the admin panel would normally flash upon first load with Blueprint, but didn't.
- Extensions with the identifier `extensions` will no longer break dashboard CSS.

#### Breaking changes

- Website URL changed from [`ptero.shop`](https://ptero.shop) to [`blueprint.zip`](https://blueprint.zip) which means some extensions might need to update the "Requires Blueprint" banner on their marketplace listings.
- Incorrect paths for `conf.yml` options will now result in a fatal error, which may break some extensions.
