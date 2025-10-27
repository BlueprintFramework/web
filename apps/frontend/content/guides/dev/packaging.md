---
title: Exporting your extension
description: Export extensions into "myextension.blueprint" files for distribution
author: Emma
category: dev
thumbnail: files.jpeg
---

## Introduction

Blueprint extensions are distributed with `{identifier}.blueprint` files. Users can upload these files to their Pterodactyl directory and install your extension through the Blueprint CLI.

## Export the extension

To turn your development files into a distributable `{identifier}.blueprint` file, we'll need to use Blueprint's CLI utility to package it.

```bash
# Exports your extension and saves the extension in your
# Pterodactyl webserver directory (e.g. /var/www/pterodactyl)
blueprint -export
```

### Optionally generate a download link

Blueprint can generate a temporary download link for your extension for easier exporting. To generate a download link, use the `expose` argument.

```bash
# Exports your extension, saves the extension in your
# Pterodactyl webserver directory and generates a temporary
# download link
blueprint -export expose
```

## Test the extension

Testing is important! You should always check if an extension still works as expected, especially when using export scripts. Install your extension, test it on your panel and finally, if all is right, distribute it.

```bash
# Make sure to remove your extension beforehand, if it is
# installed.
blueprint -remove myextension

# Install the myextension.blueprint file once again, which
# should now exist in your Pterodactyl panel.
blueprint -install myextension
```

## That's it!

You can now distribute your `{identifier}.blueprint` file. There are a few marketplaces that have specific categories/tags for Blueprint extensions. If your extension is open-source, add the [blueprint-extension](https://github.com/topics/blueprint-extension) tag to your repository!
