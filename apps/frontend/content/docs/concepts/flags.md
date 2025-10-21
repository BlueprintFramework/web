---
title: Flags
description: Feature-specific and experimental extension flags
category: concepts
---

## What are feature-flags?

Blueprint allows extensions to control certain features through feature-flags, defined in the `info.flags` [conf.yml](/docs/configs/confyml#infoflags) option. These flags can range from toggling beta features to skipping install steps for extensions.

### Why not have configuration options instead?

Feature-flags are usually less vital to extension functionality over configuration options or bindings. Phasing out older flags is in this case easier, as well as adding new ones and changing existing ones.

## Setting feature-flags

The `info.flags` [conf.yml](/docs/configs/confyml#infoflags) option expects a comma-separated list of flags. You simply add a flag to the list, and Blueprint will behave differently depending on what the flag does.

::card
Flags not known to Blueprint are ignored and will not change behavior. Each flag is matched from the flags list, and only the last flag of the list does not require a comma.
::

```yaml [conf.yml]
info:
  # a list of comma-separated feature-flags
  flags: 'flag1, flag2, flag3'
```

## Available feature-flags

### Placeholders

Feature-flags for Blueprint's [extension placeholders](/docs/concepts/placeholders).

#### `ignorePlaceholders`

Disable placeholders altogether.

#### `forceLegacyPlaceholders`

Use Blueprint's (deprecated) legacy placeholders. This flag will automatically be applied if your extension is built for any `alpha` or `indev` Blueprint release.

### Development

Developer feature-flags only apply to developer commands.

#### `developerIgnoreInstallScript`

Skip running the extension's [install script](/docs/concepts/scripts) when the extension is installed through developer commands.

#### `developerIgnoreRebuild`

Skip rebuilding frontend assets when the extension is installed through developer commands. Asset rebuilds are only triggered when Blueprint determines that your extension may require one.

#### `developerKeepApplicationCache`

Skip flushing the application's cache when the extension is installed through developer commands.

#### `developerEscalateInstallScript`

Blueprint runs [scripts](/docs/concepts/scripts) as the webserver user by default. For extension development only, scripts can be ran with administrator privileges.

#### `developerEscalateExportScript`

Blueprint runs [scripts](/docs/concepts/scripts) as the webserver user by default, but your export script may need additional permissions. This flag will provide administrator privileges to your extension's export script.

### Deprecated

Deprecated feature-flags may be ignored by Blueprint and should not be used.

#### ~~`hasInstallScript`~~ (deprecated)

Blueprint will look for a file called `install.sh`, relative to the extension's data directory, during extension installation. If present, Blueprint will run the script. **As of beta-2024-12, Blueprint no longer requires a flag to enable install scripts.**

#### ~~`hasRemovalScript`~~ (deprecated)

Blueprint will look for a file called `remove.sh`, relative to the extension's data directory, during extension removal. If present, Blueprint will run the script. **As of beta-2024-12, Blueprint no longer requires a flag to enable remove scripts.**

#### ~~`hasExportScript`~~ (deprecated)

Blueprint will look for a file called `export.sh`, relative to the extension's data directory, during extension export/packaging. If present, Blueprint will run the script. **As of beta-2024-12, Blueprint no longer requires a flag to enable export scripts.**

#### ~~`developerForceMigrate`~~ (deprecated)

Forcefully migrate the database non-interactively when installing your extension through developer commands. **As of beta-2024-12, Blueprint no longer requires interaction for database migrations.**
