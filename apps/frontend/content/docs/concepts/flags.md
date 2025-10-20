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

Feature-flags that impact your extension in production and development.

| Flag                      | Description                |
| ------------------------- | -------------------------- |
| `ignorePlaceholders`      | Skip applying placeholders |
| `forceLegacyPlaceholders` | Use legacy placeholders    |

### Developer flags

Developer feature-flags only apply to developer commands.

| Flag                             | Description                                                                                   |
| -------------------------------- | --------------------------------------------------------------------------------------------- |
| `developerIgnoreInstallScript`   | Ignore the custom extension installation script.                                              |
| `developerIgnoreRebuild`         | Skip rebuilding panel assets on installation.                                                 |
| `developerKeepApplicationCache`  | Skip flushing the application's cache on installation.                                        |
| `developerEscalateInstallScript` | Run install scripts with root permissions instead of running them through the webserver user. |
| `developerEscalateExportScript`  | Run export scripts with root permissions instead of running them through the webserver user.  |
