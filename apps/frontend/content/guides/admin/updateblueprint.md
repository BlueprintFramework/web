---
title: Update Blueprint
description: Switch to the latest version of Blueprint within minutes
author: Emma
category: admin
thumbnail: 007.jpeg
---

::card
This guide only goes through updating Blueprint. If you'd like to update Blueprint and Pterodactyl together, refer to [Update Pterodactyl](/guides/admin/updatepanel).
::

## Updating Blueprint to the latest release

Run `blueprint -upgrade` to update Blueprint to the latest stable release. This will print a few warnings, don't worry though, your extension settings won't be lost.

```bash
blueprint -upgrade
```

## Updating Blueprint to the latest commit

Living on the edge? The following command will update Blueprint to the latest GitHub commit.

::card
Latest commit is for development purposes only. No support will be provided and things **will break**.
::

```bash
blueprint -upgrade remote
# or
blueprint -upgrade remote blueprintframework/framework
```

## Updating to the latest commit of a fork

Using a framework fork? You can switch to it by using the `remote` argument on `blueprint -upgrade`. At this time, Blueprint expects a GitHub repository name as argument.

```bash
blueprint -upgrade remote organization/repository
```
