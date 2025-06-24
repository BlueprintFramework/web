---
title: Manage extensions
description: Install, remove, and configure Blueprint extensions
category: admin
thumbnail: 002.jpeg
---

Blueprint extensions must be installed, updated, built and removed via the command line. Shell access is required to perform these actions.

## Install Extensions

Only use extensions from trusted sources such as Blueprint's extension repository, sourceXchange or BuiltByBit. Third-party sources may contain unverified or malicious code.

To install a Blueprint extension, move the `.blueprint` file into your Pterodactyl root directory, usually `/var/www/pterodactyl`.

Then run the following command in your terminal:

```bash
blueprint -install [extension]
```

Replace `(extension)` with the name of the extension or the exact filename of the `.blueprint` file. This will trigger the installation process.

## Remove Extensions

To uninstall a Blueprint extension, run the following command:

```bash
blueprint -remove [extension]
```

Replace `(extension)` with the name of the extension you want to remove. This will safely remove the extension and its associated components.

## Manage extension specific settings

Once an extension is installed, you can configure its settings directly from the Pterodactyl admin dashboard.

1.  Log in to the admin interface.

2.  Click on the puzzle piece icon in the top-right corner to open the Extensions Overview.

3.  From here, select the extension you wish to manage.
    This will open a configuration view where you can adjust extension-specific options.

Settings interfaces are defined by the extension developer and may vary depending on the extension. Some even don't have settings at all.
