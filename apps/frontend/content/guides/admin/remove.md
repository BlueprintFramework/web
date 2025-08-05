---
title: Remove Blueprint
description: Uninstall Blueprint from your machine
author: Emma
category: admin
thumbnail: old_man_yells_at_blueprint.jpg
---

THIS GUIDE IS WORK IN PROGRESS

::card
Before going through this guide, be absolutely sure you've backed up your `.env` file. If you lose the `APP_KEY` stored in your `.env`, **you'll lose all panel data**.
::

## Remove Pterodactyl files

Blueprint and extensions edit Pterodactyl. To get rid of these, we'll have to wipe the Pterodactyl webserver directory.

```bash
# Remove all (except hidden) files
rm -r /var/www/pterodactyl/[!.]*
```
