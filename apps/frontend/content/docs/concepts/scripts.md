---
title: Extension scripts
description: Make out-of-scope modifications to the Pterodactyl panel
category: concepts
---

## Introduction

Sometimes your extension may require to hook into or create a file Blueprint doesn't give you access to through extension APIs. In that case, you have to patch files yourself when your extension is installed, removed and updated. You can do so through scripts.

## Writing extension scripts

Extension scripts live in the root path of the `data.directory` [conf.yml bind](/docs/configs/confyml#datadirectory).

The extension install script is called `install.sh`, update script `update.sh`, remove script `remove.sh` and export script `export.sh`.

## Environment variables

Blueprint exposes environment variables to extension scripts. Depending on the script type, the variables in this list can be used.

| Variable                 | Description                                                                                                                |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| `$ENGINE`                | Codename of the engine handling the extension and matches the `{engine}` [placeholder](/docs/concepts/placeholders#engine) |
| `$EXTENSION_IDENTIFIER`  | Extension identifier (`info.identifier` in [conf.yml](/docs/configs/confyml#infoidentifier-required))                      |
| `$EXTENSION_VERSION`     | Extension version (`info.version` in [conf.yml](/docs/configs/confyml#infoversion-required))                               |
| `$EXTENSION_TARGET`      | Extension target version (`info.target` in [conf.yml](/docs/configs/confyml#infotarget-required))                          |
| `$PTERODACTYL_DIRECTORY` | Pterodactyl webserver directory                                                                                            |
| `$BLUEPRINT_VERSION`     | Blueprint version installed to the panel                                                                                   |

### Limited availability

Select environment variables are limited to select script types.

| Variable               | Description                                                                                | Availability                 |
| ---------------------- | ------------------------------------------------------------------------------------------ | ---------------------------- |
| `$BLUEPRINT_DEVELOPER` | `true` if extension is being installed through developer commands, otherwise `false`       | `install.sh` and `update.sh` |
| `$BLUEPRINT_TMP`       | Path to where the extracted extension files are currently located and handled by Blueprint | `update.sh` and `export.sh`  |

## Important considerations

What differenciates Blueprint extensions from standalone modifications the most is the focus on compatibility. You as a developer are expected to build scripts with compatibility as a focus, extensively test your scripts and take conflicts seriously. These restrictions apply to scripts themselves and any (if applicable) other files being ran by the extension.

### Script timing

Especially during extension installation, it's important to know _when_ extension scripts get ran. Below is a quick rundown of script timing and potential conditions.

| Script type  | Runs when?                                                                                                                                                                        |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `install.sh` | During the final part of extension installation, before rebuilding dashboard frontend assets (if applicable)                                                                      |
| `update.sh`  | During the initial part of extension installation, where Blueprint determines if the extension is already installed, and if so, runs the extension's update script (if available) |
| `remove.sh`  | During the initial part of extension removal, where Blueprint is yet to undo any changes made by the extension                                                                    |
| `export.sh`  | During the final part of extension exporting, right before packaging the extension to a `{identifier}.blueprint` file                                                             |

It is important to note that Blueprint does not run the `update.sh` script from an extension's updated `{identifier}.blueprint` file. Instead, Blueprint runs the `update.sh` file of the pre-updated extension.

### Editing files

**Scripts shouldn't overwrite/replace existing files**, instead, they should replace/append strings existent in out-of-scope files.

#### What you should do

You should use tools such as `sed` to search and replace strings to edit files. This tactic should be used for both adding modifications and removing them.

Here is an example of how `sed` can be used for editing files. For more information, check out "[sed, a stream editor](https://www.gnu.org/software/sed/manual/sed.html)".

```bash [install.sh]
# Replaces foo with bar in the foo.txt file
sed -i "s/foo/bar/g" $PTERODACTYL_DIRECTORY/foo.txt
```

Then upon extension removal, we undo the changes.

```bash [remove.sh]
# Replaces foo back to bar in the foo.txt file
sed -i "s/bar/foo/g" $PTERODACTYL_DIRECTORY/foo.txt
```

#### What you shouldn't do

Replacing files, overwriting files, moving files around, replacing complete file content and adding file content based on writing to a specific "file position" are all examples of scripts shouldn't do.

::card
If you are missing specific extension APIs that are preventing you from properly writing an extension, please propose them to us by making a [GitHub issue in BlueprintFramework/framework](https://github.com/BlueprintFramework/framework/issues/new/choose).
::

### Safety and privacy concerns

Extensions shouldn't make calls to internet services through scripts and be installable, removable and updatable without an internet connection.

If your extension specifically requires calls to internet services through scripts to function, users should be prompted to "approve" the request beforehand and be shown the hostname.

#### Obfuscation

No. Extension scripts should not obfuscated by any means, for any purpose.

#### Remote scripts

Neither. Extensions scripts should by no means run anything sourced from a remote endpoint, even when given explicit user approval.

### What happens in Pterodactyl, stays in Pterodactyl

Do not create, modify or alter anything outside of the Pterodactyl directory, with the only exception being `/tmp`.

For referencing Pterodactyl's location, you can use the `$PTERODACTYL_DIRECTORY` environment variable or the `{root}` [placeholder](/docs/concepts/placeholders#root).

### Placeholder limitations

Placeholder availability depends on the script. Below is a table showing a rundown of placeholder support per script.

| Script type  | Placeholder support                                                           |
| ------------ | ----------------------------------------------------------------------------- |
| `install.sh` | Full support                                                                  |
| `update.sh`  | Partial support. Relevant to previous extension install; data may be outdated |
| `remove.sh`  | Full support                                                                  |
| `export.sh`  | No support                                                                    |

Placeholders are not available in `export.sh`. They are available in `update.sh`. However, keep in mind that placeholders in that file were applied during the previous extension installation.

### Export scripts

Export scripts are for developers to simplify extension packaging. As long as you prevent yourself from destroying your own operating system, you are generally fine.

That said, if you are about to export an extension that has a custom `export.sh` script, read through it before running it.
