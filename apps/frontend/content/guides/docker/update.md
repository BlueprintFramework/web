---
title: Update Blueprint Docker
description: Update your Blueprint Docker instance to the latest version
author:
category: docker
thumbnail: up.jpg
order:
---

# Updating Blueprint in Docker
Remember, always [create a backup](<https://github.com/BlueprintFramework/docker?tab=readme-ov-file#first-well-install-restic-to-handle-backups>) before updates
## Option 1: Only update Blueprint
```bash
blueprint -upgrade
```

## Option 2: Update both Blueprint and Pterodactyl Panel
This guide operates under the assumption that individual extension/theme authors have chosen to store any persistent data such as settings in the database or browser cookies. If they have not done this... there isn't any specific place extension data is meant to be stored, so the data could be anywhere. You'll need to ask them if there is any persistent data stored anywhere that you have to back up before updating.<br>
Go to the directory of your docker-compose.yml file<br>
Change the tag in your panel's image (i.e. to upgrade from **v1.11.10** to **v1.11.11**, you would change ``ghcr.io/blueprintframework/blueprint:v1.11.10`` to ``ghcr.io/blueprintframework/blueprint:v1.11.11``.
```bash
  docker compose pull
```
```bash
  docker compose down -v panel # ⚠ DESTRUCTIVE ACTION
```
The -v tells it to delete any named volumes, i.e. the app volume we use. It will not delete data from bind-mounts. This way the new image's app volume can take its place.
```bash
  docker compose up -d
```
Lastly, install your extensions again. You can reinstall all of the extensions in your extensions folder with
```bash
blueprint -i *.blueprint
```
If any of your extensions' settings are gone after this step, restore from your backup and ask the author of those extensions where persistent data is stored so you can back it up and restore it after each update.