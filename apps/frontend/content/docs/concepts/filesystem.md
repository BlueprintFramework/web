---
title: Storing files
description: Handle and manage (user-uploaded) files
category: concepts
---

::card
This document is a simplified and Blueprint-oriented version of [Laravel's File Storage documentation](https://laravel.com/docs/10.x/filesystem). We heavily recommend learning more about Laravel's filesystems before making use of this feature.
::

## Filesystems

Each extension automatically gets 2 filesystems, `blueprint:identifier` and `blueprint_private:identifier`. These serve two different purposes, so it's important that you use the right one.

| Disk                           | Description                                                                                                        |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `blueprint:identifier`         | Files are stored in `storage/extensions/identifier/` and can be visited by anyone from `domain.ext/fs/identifier/` |
| `blueprint_private:identifier` | Files are stored in your extension's private directory and cannot be visited publicly                              |

Keep in mind that `blueprint:identifier`'s files can be visited by **anyone that can visit your panel**, whether they are logged in or not.
