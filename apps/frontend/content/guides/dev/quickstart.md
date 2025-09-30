---
title: Quick start
description: Make your very own Blueprint extension
author: Emma
category: dev
thumbnail: code.jpeg
order: -1
---

::card
Developer mode has to be turned on in your admin panel before you can run developer commands. Navigate to `/admin/extensions`, select "Blueprint" and set `developer` to `true`. You only have to do this once.
::

## Initialize your extension

Run the `blueprint -init` command to create your new extension. This command allows you to start from a pre-made template, fill out extension information and autogenerate types.

```bash
# Create a new extension
blueprint -init
```

Most extension information options can be changed quite easily later.

## Make your first edit

Blueprint created your extension in `.blueprint/dev`, relative to your Pterodactyl directory (which is usually `/var/www/pterodactyl`, making it `/var/www/pterodactyl/.blueprint/dev`).

```sh
# Navigate into Blueprint's dev directory
cd /var/www/pterodactyl/.blueprint/dev
```

### Change the description

Try changing your extension's description. Open the `conf.yml` (`.blueprint/dev/conf.yml`) file and adjust the `info.description` value.

```diff
info:
- description: "This is my old description :c"
+ description: "This is my new description :)"
```

## Applying changes

Finally, apply your changes with the `blueprint -build` command. You can run this command every time you make a change to see your extension in action on your panel!

```sh
# Apply extension changes
blueprint -build
```

## What now?

There are a lot of resources to help you get started - from our developer community to guides like these, we've got them. Here are a few that might come in useful:

- Guides for extension development (like this one!) - [check them out here](/guides/list/dev).
- [Documentation](/docs) to help you understand Blueprint.
- Chat with more extension developers in our [Discord community](https://discord.com/servers/blueprint-1063548024825057451).
- Showcase extensions or get support on [GitHub Discussions](https://github.com/orgs/BlueprintFramework/discussions).
