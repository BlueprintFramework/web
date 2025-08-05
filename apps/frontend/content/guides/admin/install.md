---
title: Install Blueprint
description: Get Blueprint installed onto your Pterodactyl panel
author: Emma
category: admin
thumbnail: 001.jpeg
order: -1
---

## Define Pterodactyl directory

Define your Pterodactyl directory, we'll use this later in the guide. If your Pterodactyl webserver **is not** located in this directory, change the path.

```bash
# Use this command to set a $PTERODACTYL_DIRECTORY variable
# for use later in this guide.
export PTERODACTYL_DIRECTORY=/var/www/pterodactyl
```

## Dependencies

Blueprint, just like Pterodactyl, relies on a few dependencies to function. Install them through the commands below.

```bash
# Install dependencies
sudo apt install -y ca-certificates curl git gnupg unzip wget zip

# Add Node.js apt repository
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_20.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list
sudo apt update
sudo apt install -y nodejs

# Install yarn
npm i -g yarn
yarn install
```

## Download and install Blueprint

Download the latest version of Blueprint onto your server by either downloading the latest release [from GitHub](https://github.com/BlueprintFramework/framework/releases/latest) or running the commands below (which will save the file as `release.zip` and extract it).

```bash
# Navigate to your Pterodactyl directory
cd $PTERODACTYL_DIRECTORY

# Download and unzip Blueprint's latest release
wget "$(curl -s https://api.github.com/repos/BlueprintFramework/framework/releases/latest | grep 'browser_download_url' | cut -d '"' -f 4)" -O $PTERODACTYL_DIRECTORY/release.zip
unzip -o release.zip
```

## Configure Blueprint

This step allows Blueprint to function and know where itself and Pterodactyl are located and which permissions to use. Create a file called `.blueprintrc` inside of your Pterodactyl directory to begin.

```bash
# Creates a .blueprintrc file in your Pterodactyl directory
touch $PTERODACTYL_DIRECTORY/.blueprintrc
```

Modify the `$WEBUSER`, `$USERSHELL` and `$PERMISSIONS` values to match your environment. Provided below is the standard configuration for Debian-based systems, but you might need to make your own modifications.

```bash
# Writes data to your .blueprintrc file
echo \
'WEBUSER="www-data";
OWNERSHIP="www-data:www-data";
USERSHELL="/bin/bash";' > $PTERODACTYL_DIRECTORY/.blueprintrc
```

## Run Blueprint

All that's left is giving `blueprint.sh` execute permissions and running it. Blueprint will then do the necessary commands to operate correctly automatically.

```bash
# Give blueprint.sh execute permissions
chmod +x $PTERODACTYL_DIRECTORY/blueprint.sh

# Run blueprint.sh
bash $PTERODACTYL_DIRECTORY/blueprint.sh
```

Optionally you can enable Bash autocompletion by adding `source blueprint;` into your `.bashrc` (or `.zshrc` when using ZSH).

## That's it!

You've completed the Blueprint installation guide! [Browse our extensions list](/browse) or [check out another guide](/guides).
