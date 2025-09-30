---
title: Admin page
description: Create your extension's admin page
author: Emma
category: dev
thumbnail: adminpage.jpeg
order:
unlisted: false
---

## Introduction

Every extension has an admin page (or better said, admin view). It's the preferred way for administrators to configure extensions and is linked from Blueprint's extensions page.

## Create a view

Start by creating a `view.blade.php` file, then bind it to `admin.view` in your [conf.yml](/docs/configs/confyml#adminview-required) configuration.

```yaml [conf.yml]
admin:
  # Bind the view.blade.php file to admin.view
  view: 'view.blade.php'
```

..to be continued
