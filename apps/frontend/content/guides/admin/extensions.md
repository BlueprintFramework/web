---
title: Manage extensions
description: Install, remove, and configure Blueprint extensions
category: admin
thumbnail: 002.jpeg
---

## Installing an extension

To install a Blueprint extension, move the `identifier.blueprint` file into your Pterodactyl webserver directory (usually `/var/www/pterodactyl`) and install them through the following command:

```bash
blueprint -install identifier           # this works
blueprint -install identifier.blueprint # this also works!!
```

## Removing an extension

Uninstalling extensions is dead-simple. Tell our handy CLI which extension you'd like to remove and poof, away it goes.

```bash
blueprint -remove identifier           # <- do this
blueprint -remove identifier.blueprint # or this ;)
```

## Configuring extensions

Got your extensions installed? Sweet, time to set them up! On your admin panel, locate Blueprint's "Extensions" page and click on the extension you'd like to manage. All extensions have their own admin page, some have settings, some don't.

![](/img/guides/extensions.jpeg)

On extension admin pages you can also find Blueprint's settings overlay. With the settings overlay, you can configure permissions for each extension.

![](/img/guides/permissions.jpeg)
