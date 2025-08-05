---
title: Packaging extensions
description: Export extensions into ".blueprint" files for distribution
author: Emma
category: dev
thumbnail: files.jpeg
order:
---

## Export your extension with the CLI

To turn your development files into a distributable `identifier.blueprint` file, we'll need to use Blueprint's CLI utility to package it.

```bash
# Exports your extension and saves the extension in your
# Pterodactyl webserver directory (e.g. /var/www/pterodactyl)
blueprint -export
```

### Generate a download link

Blueprint can generate a temporary download link for your extension for easier exporting. To generate a download link, use the `expose` argument.

```bash
# Exports your extension, saves the extension in your
# Pterodactyl webserver directory and generates a temporary
# download link
blueprint -export expose
```

## Manually export your extension

Don't do this! Even though extensions are practically ZIP files, exporting manually can break your extension, in current and future releases of Blueprint.

Our CLI does more than just archiving extensions, it excludes certain directories, runs export scripts and enforces the folder structure Blueprint expects.
