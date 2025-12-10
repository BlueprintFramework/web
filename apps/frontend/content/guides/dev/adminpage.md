---
title: Your first admin page
description: Learn how to build a simple admin page for your extension
author: Emma
category: dev
thumbnail: adminpage.jpeg
order: -1
---

## Introduction

Every extension has an admin page (or better said, admin view). It's the preferred way for administrators to configure extensions and is linked from Blueprint's extensions page.

## Create an admin view

Create a file called `view.blade.php` in your development directory. The contents of this file will be displayed on your extension's admin page.

Bind it to `admin.view` in your [conf.yml](/docs/configs/confyml#adminview-required) configuration. Bindings (like this one) basically tell Blueprint what file/directory has what purpose.

```yaml [conf.yml]
admin:
  # Bind the view.blade.php file to admin.view
  view: 'view.blade.php'
```

### Adding content

You can now open up your extension's admin view (the `view.blade.php` created earlier) and add content to it.

::card
A `.blade.php` file is _basically_ just HTML, but with the ability to run PHP code and use special blade-template methods. You can read more about [Blade templates and it's directives in the Laravel documentation](https://laravel.com/docs/10.x/blade#blade-directives).
::

```html [view.blade.php]
<div style="background: black; padding: 5px;">
  <span style="color: white;"> The name of my extension is {name} </span>
</div>
```

Save the file and run `blueprint -build` to apply your changes. Open your Blueprint extension list (Admin > Extensions) in your Pterodactyl admin panel, click on your extension and see your changes in action.

![](/img/guides/simpleadminview.jpeg)
