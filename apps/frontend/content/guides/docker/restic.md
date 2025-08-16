---
title: Restic backups
description: Regularly back up your panel with Restic
author: Loki and Emma
category: docker
thumbnail: restic.jpeg
order:
---

## Install Restic

Before we can back up Blueprint Docker with Restic, we need to install it. The package name is usually `restic`, e.g.

```bash
# Ubuntu / Debian / Linux Mint
apt -y install restic

# Fedora
dnf -y install restic

# Rocky Linux / AlmaLinux / CentOS
dnf -y install epel-release && dnf -y install restic

# Arch Linux
pacman -S --noconfirm restic

# openSUSE
zypper install -n restic

# Gentoo
emerge --ask=n app-backup/restic
```

## Set up Restic

Once you've installed Restic, create a directory for our backups to live in at `/srv/backups/pterodactyl` and initialize it.

```bash
# Creates a directory for backups to live in
mkdir -p /srv/backups/pterodactyl

# Initializes Restic
restic init --repo /srv/backups/pterodactyl
```

### Create a backup script

With your directory initialized, create a backup script with the below command and give it 'execute' permission so that we can run it.

```bash
# Generates and saves $RESTIC_PASSWORD to /srv/backups/.env
echo -e "RESTIC_PASSWORD=\"$(openssl rand -base64 32)\"" >> /srv/backups/.resticenv

# Writes script to /srv/backups/backup.sh
cat <<EOF > /srv/backups/backup.sh
#!/bin/bash
source /srv/backups/.resticenv
docker compose -f /srv/pterodactyl/docker-compose.yml down panel
cd /var/lib/docker/volumes/pterodactyl_app/_data
restic backup . -r /srv/backups/pterodactyl
docker compose -f /srv/pterodactyl/docker-compose.yml up -d panel
EOF

# Gives backup.sh and .resticenv execute permissions
chmod +x /srv/backups/backup.sh /srv/backups/.resticenv
```

::card
If you ever need your automatically-generated `RESTIC_PASSWORD`, you can find it in `/srv/backups/.resticenv`.
::

### Schedule backup script

With a cronjob you can schedule backups to be made every day, automatically.

```bash
# Creates a cronjob for running the backup.sh script
(crontab -l 2>/dev/null; echo "59 23 * * * /srv/backups/backup.sh") | crontab -
```

## All done!

You've set up daily Restic backups. They are set to keep at most `30` backups at a time. You might be wondering, however, how you could restore one of those backups..

### Listing backups

To list your Restic backups, run the command shown below. Each backup has an **ID** and **Time** value. They are used for identifying a backup and telling you when a backup was taken respectively.

```bash
# Gets Restic credentials
source /srv/backups/.resticenv

# Lists Restic snapshots
restic snapshots --repo /srv/backups/pterodactyl
```

### Restoring a backup

Once you've determined which backup you'd like to restore, bring your compose stack down, clear your `pterodactyl_app` volume and restore the backup.

```bash
# Gets Restic credentials
source /srv/backups/.resticenv

# Brings your compose stack down
docker compose -f /srv/pterodactyl/docker-compose.yml down

# Clears the directory so the restoration will be clean
# WARNING: This is a destructive action
rm -rf /var/lib/docker/volumes/pterodactyl_app/_data/.[!.]* /var/lib/docker/volumes/pterodactyl_app/_data/*

# Restores a Restic backup. Make sure to replace 3xamp13 with the backup ID
restic restore 3xamp13 -r /srv/backups/pterodactyl -t /var/lib/docker/volumes/pterodactyl_app/_data

# Brings your compose stack up again
docker compose -f /srv/pterodactyl/docker-compose.yml up -d
```
