---
title: Remove Blueprint
description: Uninstall Blueprint from your machine
author: Emma
category: admin
thumbnail: old_man_yells_at_blueprint.jpg
---

::card
Before going through this guide, be absolutely sure you've backed up your `.env` file. If you lose the `APP_KEY` stored in your `.env`, **you'll lose all panel data**.
::

## Enter maintenance mode

Before removing Blueprint, you should put your panel into maintenance mode.

```bash
# Navigates to your Pterodactyl directory
cd /var/www/pterodactyl

# Puts your panel into maintenance mode
php artisan down
```

## Remove files

Blueprint and extensions edit Pterodactyl. To get rid of these, we'll have to wipe the Pterodactyl webserver directory.

```bash
# Remove all Pterodactyl webserver files except hidden ones
rm -r [!.]*

# Remove leftover Blueprint files and directories
rm -r .blueprint .blueprintrc /usr/local/bin/blueprint
```

## Restore Pterodactyl

Follow [Pterodactyl's Updating the Panel guide](https://pterodactyl.io/panel/1.0/updating.html#download-the-update) from "Download the Update" onwards.
