---
version: alpha-SLX
released: 2024-02-21T21:00:25Z
summary: Routing.. finally..
num: 7
---

#### Changes

- Add `webpack` and `react` to the required dependencies.
- Extensions with website URLs linking to certain "sourceXchange" and "BuiltByBit" domains will now show the `bx-store` icon on their admin panel website button.
- Update the entire logging system by writing a new library for it and redesigning the outputs. This includes rewritten log messages in a large amount of places.
- The `-upgrade` command's script has now been moved onto `blueprint.sh` as it was only responsible for creating a temporarily folder, pulling latest release files and moving them.
- Remove the `tools` directory as all scripts inside it have been implemented directly into `-upgrade` and `-debug`.
- Step away from the `bash_colors` and `throwError` shell-based library used by `blueprint.sh` and group multiple small libraries into one library called `misc`.
- Redesign placeholder extension icons which are used when an extension does not ship with an icon.
- Improved error validation and detection in `blueprint.sh`.
- Extensions can now create routes inside of their `Components.yml` file, allowing for adding pages to the Pterodactyl client panel.
- Massively speed up `blueprint.sh` by grouping some commands and adding a bunch of optimization.
- Improve code readability across multiple parts of `blueprint.sh`.
- Rename the `controller.php` build file to `controller.build`.
- We are stepping away from null (`NUL`) characters in build file placeholders, wrapper start/end identifiers, extension template placeholders (preperations made, not enforced until older versions become deprecated) and more.
- Made small changes to both the `admin.extensions` and `admin.extensions.blueprint` view.
- Installing and removing extensions now clears the application cache.
- Add `-add`, which is an alias of `-install`.
- Abort extension removal if argument one ends with `.blueprint`.
- `ExtensionsController` and Blueprint's database migration have been switched over to 2-space indentation.
- Admin view routes have been moved to `routes/blueprint.php`.
- Admin and dashboard wrappers have now been moved to `partials/blueprint/`.
- Webpack build progress is now shown when Blueprint needs to rebuild panel assets.

#### Bugfixes

- The `blueprint` CLI command would return exit code `1` in all cases, even successful events. This has been replaced by `0` for "SUCCESS" events, `1` for "ERROR" events, `2` for "USER-ERROR" events.
- `SubNavigation` options in `Components.yml` had an inconsistency in component placement across similar options.
- The `admin.extensions.blueprint` view had auto-complete enabled on it's input form which caused unsaved options to be cached.
- `-remove` would default every unknown response as 'yes' on it's confirmation question, which should be defaulted to 'no'.
