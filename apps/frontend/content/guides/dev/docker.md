---
title: Develop with Blueprint Docker
description: Set up Blueprint Docker for local development
author: Loki and Emma
category: dev
thumbnail: blueprintdocker.jpg
---

::card
This guide is made specifically for local extension development. If you want to use Blueprint Docker in production, follow the instructions in the [README](https://github.com/BlueprintFramework/docker#readme).
::

## Create directory and pull files

First, make a `/srv/pterodactyl` directory and `cd` into it. Then, proceed to fetch Blueprint's `docker-compose.yml` and save it in that directory.

```bash
# Creates a /srv/pterodactyl directory and navigates into it
mkdir -p /srv/pterodactyl
cd /srv/pterodactyl

# Downloads Blueprint's docker-compose.yml into the current directory
wget https://docker.bpfw.io/docker-compose.yml
```

## Environment configuration

Before we can get the stack running, you need to create and fill out a few environment variables. While you can see the full list [on the GitHub repository](https://raw.githubusercontent.com/BlueprintFramework/docker/refs/heads/Master/.env), we'll go through the options category-by-category through the next few steps.

```bash
# Creates and opens .env with nano
touch .env
nano .env
```

Now, paste in the environment variables documented below. You can functionally copy them as-is.

### Application

These environment variables relate to your Pterodactyl webserver. Adjust them if needed.

```bash [/srv/pterodactyl/.env]
# Base directory, /srv/pterodactyl by default
BASE_DIR=/srv/pterodactyl

# Name of your application, will show up on your panel and
# emails sent from it
APP_NAME="Pterodactyl"

# Fully Qualified Domain Name (FQDN) of your panel, or simply
# put, the FIRST address leading to your panel. If you're accessing
# your panel through a proxy, you'd put the proxy's endpoint. If you're
# home-hosting this, you'd put your public IP or a domain leading to it.
# If you're testing it out on your local machine only, you'd probably
# use http://localhost
FQDN="http://localhost"

# Timezone to use. List of timezones available at:
# http://php.net/manual/en/timezones.php
TIMEZONE=Europe/Amsterdam

# Application environment. Can be either 'production' or
# 'testing'
APP_ENV=production

# Port to host the panel on. To use another port, allocate
# one in your docker-compose.yml and assign it here. We recommend
# using Cloudflare DNS and enabling automatic redirection to https.
PANEL_PORT=443
```

### Captcha

Configure whether or not to show a captcha, and optionally add your api keys here.

```bash [/srv/pterodactyl/.env]
# Toggle whether to enable recaptcha or not
RECAPTCHA_ENABLED=false

# Recaptcha API keys. You can generate custom site keys at
# https://www.google.com/recaptcha/admin
RECAPTCHA_SITE_KEY=
RECAPTCHA_SECRET_KEY=
```

### Mail

Set up email for your panel, SMTP details and from which domain to send the email.

```bash [/srv/pterodactyl/.env]
# Domain name to send emails from
DOMAIN=blueprint.local

# Email driver. To effectively disable the mail feature,
# set it to 'array' instead of 'smtp'
MAIL_DRIVER=array

# SMTP details
SMTP_SERVER=
SMTP_PORT=
SMTP_ENCRYPTION=tls
SMTP_USERNAME=
SMTP_APIKEY=
```

### Wings

Define the ports used for Wings and it's SFTP server.

```bash
# Run to define $WINGS_PORT and $WINGS_SFTP_PORT variables to write
# to your .env and use later in the guide.
export WINGS_PORT=8080
export WINGS_SFTP_PORT=2022

# Write environment variables to .env
echo "WINGS_PORT=$WINGS_PORT\nWINGS_SFTP_PORT=$WINGS_SFTP_PORT" >> .env
```

::card
After you've set up Wings later in the guide, changing `WINGS_PORT` and `WINGS_SFTP_PORT` will require changing them in the panel UI and `wings/config.yml` too.
::

### Keys

Lastly, generate keys for your panel database and redis/valkey.

```bash
echo "MARIADB_ROOT_PASS=\"$(openssl rand -base64 32)\"" >> .env
echo "MARIADB_USER_PASS=\"$(openssl rand -base64 32)\"" >> .env
echo "VALKEY_PASS=\"$(openssl rand -base64 32)\"" >> .env
echo "HASH_SALT=\"$(openssl rand -base64 32)\"" >> .env
```

## Start the stack

Once your done filling out everything, start your stack through Docker Compose.

```bash
# Starts the stack
docker compose up -d
```

## Connect Wings

Before we can _run anything_ in Pterodactyl, we'll have to "create a node", where your node in this case is Wings. Before we can do that, though, we'll have to make a location too.

### Create a new location

The following command creates a location on your panel. If you want to, you can adjust the `--short` and `--long` arguments to a more descriptive location.

```bash
# Creates a location called 'earth' with description
# 'Somewhere on planet earth'
docker compose exec panel php artisan p:location:make \
  --short=earth \
  --long="Somewhere on planet earth"
```

Since this is our first location, it will have the ID of `1`.

### Create a new node

With our newly created location, we can use the following command to create a node.

::card
In this step, we are using the `$WINGS_PORT` and `$WINGS_SFTP_PORT` environment variables defined earlier. If these variables are no longer in your local shell environment, grab them from your `.env` and replace them with the right ports inside of the command below.
::

```bash
# Define your FQDN and URL scheme for Wings to use. These options
# are similar to the one you defined for the panel earlier.
#
# This is the FIRST address leading to your Wings node. If Wings is
# behind a proxy, use the proxy's domain name.
export $WINGS_SCHEME="http" #Can be either http or https
export $WINGS_FQDN="localhost"

# Creates a new node on the panel
docker compose exec panel php artisan p:node:make \
  --name="Node" \
  --description="My awesome node" \
  --locationId=1 \
  --fqdn="$WINGS_FQDN" \
  --public=1 \
  --scheme="$WINGS_SCHEME" \
  --proxy=0 \
  --maintenance=0 \
  --maxMemory=4096 \
  --overallocateMemory=0 \
  --maxDisk=10240 \
  --overallocateDisk=0 \
  --uploadSize=100 \
  --daemonListeningPort=$WINGS_PORT \
  --daemonSFTPPort=$WINGS_SFTP_PORT \
  --daemonBase=/srv/pterodactyl/wings/pterodactyl
```

### Configure Wings

With the node defined in Pterodactyl, the panel has now generated a Wings configuration for us. We'll need to write that configuration to `wings/config.yml`.

```bash
# Write Wings configuration for node 1 to wings/config.yml
docker compose exec panel php artisan p:node:configuration 1 > wings/config.yml

# Restart Wings
docker compose restart wings
```

With Wings restarted, open up your `wings/config.yml` file, it should have a few additional options to mess with. Let's focus on the directory definitions, since those need to be adjusted to your Docker environment.
Change the paths as you see them changed below.

```diff [/srv/pterodactyl/wings/config.yml]
system:
+ root_directory: /srv/pterodactyl
+ log_directory: /srv/pterodactyl/wings/logs
+ archive_directory: /srv/pterodactyl/archives
+ backup_directory: /srv/pterodactyl/backups
- root_directory: /var/lib/pterodactyl
- log_directory: /var/log/pterodactyl
- archive_directory: /var/lib/pterodactyl/archives
- backup_directory: /var/lib/pterodactyl/backups
```

## Alias Blueprint command

To use Blueprint simply by running the `blueprint` command on your host machine, create an alias.

```bash
# Alias Blueprint in your bashrc
echo 'alias blueprint="docker compose -f /srv/pterodactyl/docker-compose.yml exec panel blueprint"' >> ~/.bashrc

# Source your .bashrc
source ~/.bashrc

# Test the Blueprint command. This should output the installed
# Blueprint version
blueprint -v
```

::card
Got Blueprint working but stuck wondering how you should go about installing extensions? You can find the answer in the [Manage extensions guide](/guides/admin/extensions#installing-an-extension).
::

## Create your first user

To actually use your panel, you want to create a user. Use the command shown below to make one.

```bash
# Creates a user and prints out the password
docker compose exec panel php artisan p:user:make \
  --email="user@example.com" \
  --username="your-awesome-username" \
  --name-first="Jane" \
  --name-last="Doe" \
  --admin=1
```

## That's it!

You've set up Blueprint Docker for local extension development. Initialize an extension and start creating!
