---
title: Update Pterodactyl
description: Update Pterodactyl and Blueprint to the latest version
author: Emma
category: admin
thumbnail: 006.jpeg
---

## Enter maintenance mode

Before updating Pterodactyl, you should always put it into maintenance mode. Not doing so can cause users stumbling onto unexpected errors and ensures everything can be updated before users encounter potentially new features.

```bash
# Navigate to your Pterodactyl directory
cd /var/www/pterodactyl

# Puts your panel into maintenance mode
php artisan down
```

## Download the latest Pterodactyl release

First, download Pterodactyl's latest release from GitHub.

```bash
# Downloads, saves and unpacks Pterodactyl's latest release archive
curl -L https://github.com/pterodactyl/panel/releases/latest/download/panel.tar.gz | tar -xzv

# Sets the correct permissions on the cache and storage directories
chmod -R 755 storage/* bootstrap/cache
```

## Update Pterodactyl's dependencies

After you've downloaded all of the new files you will need to upgrade the core components of the panel. To do this, simply run the commands below and follow any prompts.

```bash
# Upgrades Pterodactyl's core panel components
composer install --no-dev --optimize-autoloader
```

## Database updates

Update your database schema for the newest version of Pterodactyl. Running the command below will update the schema and updates the default eggs. If you've made any changes to Pterodactyl's core eggs, this command will override those.

```bash
# Updates Pterodactyl's database schema
php artisan migrate --seed --force
```

## Update Blueprint

::card
You can learn more about updating Blueprint in our [Update Blueprint guide](/guides/admin/updateblueprint).
::

Use `blueprint -upgrade` command to apply the latest Blueprint release release onto your panel. This command also restores changes that were overwritten by updating Pterodactyl.

```bash
# Updates Blueprint to the latest release
blueprint -upgrade
```

## Exit maintenance mode

Finally, use the command below to make sure you've exited maintenance mode.

```bash
# Puts your panel back into production
php artisan up
```
