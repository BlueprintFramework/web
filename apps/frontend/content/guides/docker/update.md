---
title: Update Blueprint Docker
description: Update your Blueprint Docker instance to the latest version
author: Loki and Emma
category: docker
thumbnail: up.jpg
order:
---

## Update Blueprint only

::card
You can learn more about updating Blueprint in our [Update Blueprint guide](/guides/admin/updateblueprint).
::

Use the `blueprint -upgrade` command to apply the latest Blueprint release release onto your panel. This only updates Blueprint, not the panel.

```bash
# Update Blueprint to the latest release
blueprint -upgrade
```

## Update both Blueprint and Pterodactyl

::card
This guide operates under the assumption that individual extension/theme authors have chosen to store any persistent data such as settings in the database or browser cookies. If they have not done this.. there isn't any specific place extension data is meant to be stored, so the data could be anywhere. You'll need to ask them if there is any persistent data stored anywhere that you have to back up before updating.
::

### Navigate into Pterodactyl

Navigate into your Pterodactyl directory, which is usually `/srv/pterodactyl` on Blueprint Docker installations.

```bash
# Navigates into the /srv/pterodactyl directory
cd /srv/pterodactyl
```

### Update release tag

Open up your `docker-compose.yml` file.

```bash
# Opens docker-compose.yml with nano
nano docker-compose.yml
```

Under `services.panel.image`, replace the `[old-version]` with the `[new-version]`.

For example, if you were updating from **v1.11.10** to **v1.11.11**, you would change `ghcr.io/blueprintframework/blueprint:v1.11.10` to `ghcr.io/blueprintframework/blueprint:v1.11.11`.

```diff
services:
  panel:
-   image: ghcr.io/blueprintframework/blueprint:v[old-version]
+   image: ghcr.io/blueprintframework/blueprint:v[new-version]
```

Save your changes and proceed to the next step.

### Pull image

Use the `docker compose pull` command to pull the latest Docker images.

```bash
# Pull docker images
docker compose pull
```

### Bring the stack down

The following command brings the compose stack down. The `-v` argument tells it to delete any named volumes, i.e. the app volume we use. It will not delete data from bind-mounts. This way the new image's app volume can take it's place.

```bash
# Bring the compose stack down and delete volumes
docker compose down -v panel # ⚠ DESTRUCTIVE ACTION
```

### Start the stack

Now that we've pulled the new image and brought the stack down, bring it back up with the new images.

```bash
# Bring the start back up again
docker compose up -d
```

### Reinstall extensions

If you had any extensions installed, you may have to reinstall them.

```bash
# Install extensions through the Blueprint CLI
blueprint -install myextension anotherextension
```

### All done!

You've now updated your Blueprint Docker installation.
