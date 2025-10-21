---
title: Placeholders
description: Automatically replace strings with data
category: concepts
---

## Introduction

Placeholders are strings developers put in their extensions, these can range from generating a random number to autofilling your extension identifier. They are quite useful, work in (almost) every file and can help your extension work on all kinds of installations.

### Why?

Extension placeholders make it easy to prefill (static) data anywhere in your extension. What would otherwise be only accessible in certain languages (and requiring Blueprint to ship a library for each one), is available in any text file across your extension.

## Escaping placeholders

There are times where you need to use `{name}` (or any other placeholder) in your extension, but don't want it to get replaced by Blueprint. You can add a `!` in front of the placeholder to escape it, like demonstrated below.

```bash
echo "You don't know my !{name}"
# ..becomes..
echo "You don't know my {name}"
```

### Completely disable placeholders

To fully disable Blueprint's placeholders add the `ignorePlaceholders` flag to your extension's config.

```yaml [conf.yml]
info:
  flags: 'ignorePlaceholders'
```

## Placeholders (and examples)

### Configuration values

This placeholder group contains extension-config-related values.

#### Identifier

Returns the extension's identifier. This identifier can be configured in your [conf.yml](/docs/configs/confyml) and matches the identifier Blueprint uses for paths/variables related to your extension.

| Placeholder     | Output      | Modifier                        |
| --------------- | ----------- | ------------------------------- |
| `{identifier}`  | myextension |                                 |
| `{identifier^}` | Myextension | Capitalizes the first character |
| `{identifier!}` | MYEXTENSION | Capitalizes all characters      |

#### Name

Returns the extension's display name which you configured in your [conf.yml](/docs/configs/confyml). Extension display names are not limited to a limited set of characters, so parse wisely.

| Placeholder | Output       | Modifier                   |
| ----------- | ------------ | -------------------------- |
| `{name}`    | My Extension |                            |
| `{name!}`   | MY EXTENSION | Capitalizes all characters |

#### Author

If defined in the extension's config, it will return defined author, otherwise `undefined` is returned.

| Placeholder | Output                         |
| ----------- | ------------------------------ |
| `{author}`  | Nothing to see here industries |

#### Version

Returns the extension's version.

| Placeholder | Output |
| ----------- | ------ |
| `{version}` | 1.0.3  |

### Directories

Placeholders that point towards certain directories or file paths.

#### Root

Returns the root directory of the target Pterodactyl installation, which is usually `/var/www/pterodactyl`. This placeholder has modifiers to fetch paths to specific folders.

| Placeholder     | Output                                                         | Modifier                                               |
| --------------- | -------------------------------------------------------------- | ------------------------------------------------------ |
| `{root}`        | /path/to/pterodactyl                                           |                                                        |
| `{root/public}` | /path/to/pterodactyl/.blueprint/extensions/myextension/public  | Path to public directory                               |
| `{root/data}`   | /path/to/pterodactyl/.blueprint/extensions/myextension/private | Path to data directory                                 |
| `{root/fs}`     | /path/to/pterodactyl/.blueprint/extensions/myextension/fs      | Path to [public filesystem](/docs/concepts/filesystem) |

In the example below, we're using the `{root/public}` placeholder in an [extension script](/docs/concepts/scripts) to create a file in the extension's public directory.

```bash [install.sh]
#!/bin/bash

touch "{root/public}/foo.txt"
echo "bar" > "{root/public}/foo.txt"
```

Once installed, the extension should have created a file at "domain.ext/extensions/myextension/foo.txt" with the content "bar".

#### Webroot

Returns paths relative to the website's root url. These are not local paths.

| Placeholder        | Output                     | Modifier                                                                 |
| ------------------ | -------------------------- | ------------------------------------------------------------------------ |
| `{webroot}`        | /                          |                                                                          |
| `{webroot/public}` | /extensions/myextension    | Path to exposed public directory                                         |
| `{webroot/fs}`     | /fs/extensions/myextension | Path to exposed [public filesystem](/docs/concepts/filesystem) directory |

For example, you could use the `{webroot/public}` placeholder to add an image to a page.

```html [example.html]
<!-- Embed cool_image.jpeg from the extension's public directory -->
<img src="{webroot/public}/cool_image.jpeg" />
```

### Installer

Technical details about the framework handling the extension.

#### Engine

Provides information about the engine handling the extension. Different engines using the extension standard that aren't affiliated with Blueprint are expected to replace this placeholder with their own codenames.

| Placeholder | Output   |
| ----------- | -------- |
| `{engine}`  | solstice |

#### Target

Returns the version of Blueprint the extension installed on. This placeholder **will not** return the defined extension's target version.

| Placeholder   | Output       | Modifier                                                                                      |
| ------------- | ------------ | --------------------------------------------------------------------------------------------- |
| `{target}`    | beta-2025-09 |                                                                                               |
| `{is_target}` | true         | `true` or `false` depending on if the extension's target version matches the one of Blueprint |

With this info, your extension could warn users if they aren't using the Blueprint version your extension was designed for.

```html [view.blade.php]
@if("{is_target}" != "true")
<div style="background: red; padding: 12px; color: white;">
  You are using Blueprint version {target} which this extension wasn't made for.
  Everything should still work fine, but be warned :P
</div>
@endif
```

#### Mode

Returns either `local` or `develop` depending on if the extension has been installed through a packaged `myextension.blueprint` file or developer commands.

| Placeholder | Output |
| ----------- | ------ |
| `{mode}`    | local  |

#### Timestamp

Returns a Unix timestamp that indicates when the extension was installed.

| Placeholder   | Output     |
| ------------- | ---------- |
| `{timestamp}` | 1712239495 |

### Miscellaneous

Uncategorized and advanced placeholders that do various things.

#### Filesystem

Returns the extension's filesystem name, for both public and private filesystems. Learn more about filesystems in the [Storing files document](/docs/concepts/filesystem).

| Placeholder    | Output                        | Modifier                     |
| -------------- | ----------------------------- | ---------------------------- |
| `{fs}`         | blueprint:myextension         |                              |
| `{fs/private}` | blueprint_private:myextension | Private filesystem variation |

#### Random

This placeholder will be replaced with a 'random' number. This number is determined **once per file**. Random numbers may vary in length, so parse wisely.

| Placeholder | Output |
| ----------- | ------ |
| `{random}`  | 17388  |

## Marketplace placeholders

Marketplace placeholders (or "Anti-piracy placeholders") are provided by the marketplace you plan to sell on. Some marketplaces may have issues with (or skip) `myextension.blueprint` files, please contact your marketplace if you face any issues.

- BuiltByBit has support for packaged extensions, [learn more about their placeholders here](https://builtbybit.com/wiki/anti-piracy-placeholders/).
- sourceXchange also supports packaged extensions. They provide similar placeholders to BuiltByBit's ones, but have only documented it in a [Discord announcement post](https://discord.com/channels/998177114018816000/1174812473656889444/1194653774044745768).
