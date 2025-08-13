---
title: Install Blueprint Docker
description: Get started with Blueprint Docker
author: Loki and Emma
category: docker
thumbnail: blueprintdocker.jpg
order: -1
---

## Introduction

::card

#### Supported Architectures

**AMD64** :white_check_mark:<br>
**ARM64** :white_check_mark:<br>
::

Note: While the panel and Wings images provided will run fine on Arm64, most game servers _will not_, so if you are running Wings on an Arm64 machine, that's something to be aware of.

::card

#### Wings on Raspberry Pi

Running Wings on a Raspberry Pi 4 or 5 may require additional work since Wings requires Docker cgroups, which are not present in the Ubuntu releases, only in Debian 11/12.

Install the Debian Lite OS on your Raspberry Pi and install Docker. Add the following arguments in `/boot/firmware/cmdline.txt` (or `/boot/cmdline.txt` on Debian 11)

```txt [/boot/firmware/cmdline.txt]
cgroup_memory=1 cgroup_enable=memory systemd.unified_cgroup_hierarchy=0
```

::

#### What is the difference between docker-compose.yml and classic-docker-compose.yml?

`classic-docker-compose.yml` stays as close to the stock Pterodactyl compose file as possible. This means it still has the obsolete "version" attribute, has no health checks, and does not use a .env file for configuration. This file is simpler to look at and understand, mostly because it doesn't give you the same level of control and information as the recommended docker-compose.yml file.<br>
`docker-compose.yml` (recommended) can and has been improved over time. If you are using this version, download and configure the .env file as well; most if not all configuration can be done through the .env file.

## Installation

#### Bringing your compose stack online

Download the compose file of your choice where you want your app to live (`/srv/pterodactyl` by default). If you chose docker-compose.yml, download the .env file to the same directory and configure it. If you chose classic-docker-compose.yml, configure the compose file only; you do not need the .env file.<br>
**Note: This is not a drag and drop service; you need to configure it. For example, if you're setting up a panel-only machine, remove the wings service.**<br>
When you are done configuring, `cd /srv/pterodactyl` and bring the stack up with `docker compose up -d`

#### Is this your first time running Wings inside of Docker?

One thing to be prepared for is that Wings uses the host system's Docker Engine through the mounted socket; it does not use Docker in Docker. What this means is the directory where you store your data, if you wish to customize it, must be set to the same value for both host and container in the mounts, and then you must make the values in your config.yml match; otherwise the Wings container would see one directory, then when a new container is created that isn't affected by this docker-compose.yml's mounts, it won't see the same directory. Here's an example:<br>

\- Mount in docker-compose.yml: `"${BASE_DIR}/:${BASE_DIR}/"`.<br>
\- Let's say, for the purposes of this example, that you set `BASE_DIR` in your .env file to **/srv/pterodactyl**. If you want to mount Wings server data in another location, just add any other mount, making sure both sides of the mount match. Now when you create your node, you would select somewhere inside the mount you made for `Daemon Server File Directory`, e.g. `/srv/pterodacty/wings/servers`.<br>
\- After Wings runs successfully the first time, more options will appear in your `config.yml` file. They will look like this:

```yaml
root_directory: /var/lib/pterodactyl
log_directory: /var/log/pterodactyl
data: /srv/pterodactyl/wings/servers
archive_directory: /var/lib/pterodactyl/archives
backup_directory: /var/lib/pterodactyl/backups
tmp_directory: /tmp/pterodactyl
```

As you can see, only `data` gets set to your configured location. You can make the others match by changing `/var/lib/pterodactyl` to match your base directory, again for the example `/srv/pterodactyl`. Optionally, you can change the log location too if you'd like to keep **_everything_** possible inside one directory, which is one of the benefits of using containers. Once you're done, it may look like:

```yaml
root_directory: /srv/pterodactyl
log_directory: /srv/pterodactyl/wings/logs
data: /srv/pterodactyl/wings/servers
archive_directory: /srv/pterodactyl/archives
backup_directory: /srv/pterodactyl/backups
tmp_directory: /tmp/pterodactyl
```

#### Creating your first user

`cd` into the directory containing your compose file, e.g. `cd /srv/pterodactyl`

```bash
docker compose exec panel php artisan p:user:make
```

#### Interacting with Blueprint

Setting this alias will let you follow guides written for Blueprint. You can then interact with blueprint inside your container from the blueprint command on your host system. If your compose file is not in `/srv/pterodactyl/docker-compose.yml`, change the path to where yours is.

```bash
# Set alias for current session
alias blueprint="docker compose -f /srv/pterodactyl/docker-compose.yml exec panel blueprint"
# Append to the end of your .bashrc file to make it persistent
echo 'alias blueprint="docker compose -f /srv/pterodactyl/docker-compose.yml exec panel blueprint"' >> ~/.bashrc
```
