---
title: Restic backups
description: Regularly back up your panel with Restic
author: Loki
category: docker
thumbnail: restic.jpeg
order:
---

::card
So, you installed your first extension. Congratulations! Blueprint is now keeping persistent data inside the `pterodactyl_app` volume, so you'll want to start backing that volume up regularly.
::

#### First, we'll install Restic to handle backups

Why Restic? Compression, de-duplication, and incremental backups. Save on space compared to simply archiving the directory each time.<br>
The package name is usually `restic`, e.g.
::card
**Ubuntu / Debian / Linux Mint**<br>
`sudo apt -y install restic`<br>
**Fedora**<br>
`sudo dnf -y install restic`<br>
**Rocky Linux / AlmaLinux / CentOS**<br>
`sudo dnf -y install epel-release && sudo dnf -y install restic`<br>
**Arch Linux**<br>
`sudo pacman -S --noconfirm restic`<br>
**openSUSE**<br>
`sudo zypper install -n restic`<br>
**Gentoo**<br>
`sudo emerge --ask=n app-backup/restic`<br>
::

#### Make a directory and script for backups

```bash
mkdir -p /srv/backups/pterodactyl
export RESTIC_PASSWORD="CHANGE_ME"
restic init --repo /srv/backups/pterodactyl
cat <<EOF > /srv/backups/backup.sh
#!/bin/bash
docker compose -f /srv/pterodactyl/docker-compose.yml down panel
cd /var/lib/docker/volumes/pterodactyl_app/_data
RESTIC_PASSWORD="${RESTIC_PASSWORD}" restic backup . -r /srv/backups/pterodactyl
docker compose -f /srv/pterodactyl/docker-compose.yml up -d panel
EOF
chmod +x /srv/backups/backup.sh
```

#### Set a crontab to back up your panel (choose a time when it will be least likely to be being used)

```bash
(crontab -l 2>/dev/null; echo "59 23 * * * /srv/backups/backup.sh") | crontab -
```

#### Well, great. I have daily backups now, and they're set to keep at most 30 backups at a time. How can I restore from one of them?

You can list snapshots with `restic snapshots --repo /srv/backups/pterodactyl`<br>
You're looking for a value for **ID** that looks something like `46adb587`. **Time** will be right next to each ID, so you can see what day your backups are from.

#### Once you've determined which snapshot you want to restore, stop your compose stack, restore your data, and start your stack again

```bash
docker compose -f /srv/pterodactyl/docker-compose.yml down
# Clear the directory so the restoration will be clean ⚠ DESTRUCTIVE ACTION
rm -rf /var/lib/docker/volumes/pterodactyl_app/_data/.[!.]* /var/lib/docker/volumes/pterodactyl_app/_data/*
# Remember to replace "46adb587" with your actual ID of the snapshot you want to restore
restic restore 46adb587 -r /srv/backups/pterodactyl -t /var/lib/docker/volumes/pterodactyl_app/_data
docker compose -f /srv/pterodactyl/docker-compose.yml up -d
```
