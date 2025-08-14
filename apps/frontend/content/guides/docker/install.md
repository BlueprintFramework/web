---
title: Install Blueprint Docker
description: Get started with Blueprint Docker
author: Loki and Emma
category: docker
thumbnail: blueprintdocker.jpg
order: -1
---

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

# Fully Qualified Domain Name (FQDN) of your panel, or simply
# said, your panel's website url
FQDN="https://pterodactyl.local"

# Timezone to use. List of timezones available at:
# http://php.net/manual/en/timezones.php
TIMEZONE=Europe/Amsterdam

# Application environment. Can be either 'production' or
# 'testing'
APP_ENV=production

# Port to host the panel on. To use another port, allocate
# one in your docker-compose.yml and assign it here
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
DOMAIN=example.com

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

Lastly, a few environment variables for your Wings installation.

```bash [/srv/pterodactyl/.env]
# Port to use for Wings
WINGS_PORT=8080

# Port Wings will use to listen for SFTP connections
WINGS_SFTP_PORT=2022
```

## Start the stack

Once your done filling out everything, start your stack through Docker Compose.

```bash
# Starts the stack
docker compose up -d
```

## Create your first user

To actually use your panel, you want to create a user. Use the command shown below to make one.

```bash
# Creates a user
docker compose exec panel php artisan p:user:make
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

## Connecting Wings

TO BE DOCUMENTED

This should guide the user through making a node and pasting the correct items into the wings config

## That's it!

You've completed the Blueprint Docker installation guide! [Browse our extensions list](/browse) or [check out other related guides](/guides/list/docker).
