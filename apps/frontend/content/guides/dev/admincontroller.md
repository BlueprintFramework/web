---
title: Custom admin page controller
description: Render your admin view using a custom controller
author: Emma
category: dev
thumbnail: controller.jpeg
---

## Introduction

Using the `admin.controller` [conf.yml bind](/docs/configs/confyml#admincontroller) you can add additional logic to how your `admin.view` is rendered, as well as how additional HTTP methods are handled.

Admin views are always rendered through controllers. Unless you are shipping your own custom controller (which you will when following this guide), your extension will use the default one shipped by us.

## Create the controller

Create a file called `controller.php`, and bind it to `admin.controller` in your [conf.yml](/docs/configs/confyml#admincontroller) configuration.

```yaml [conf.yml]
admin:
  controller: 'controller.php'
```

Then, add the code below to your `controller.php` file.

```php [controller.php]
<?php

// Define namespace. This is the namespace needed for admin controllers.
// {identifier} automatically gets replaced with your extension's
// identifier upon extension installation.
namespace Pterodactyl\Http\Controllers\Admin\Extensions\{identifier};

// Import the controller class.
use Pterodactyl\Http\Controllers\Controller;

// Register the extension-specific controller class.
class {identifier}ExtensionController extends Controller {
  // The index() function gets called upon a GET request.
  public function index() {
    // Respond with plain text.
    return "hello from {identifier}'s custom controller"
  }
}
```

Open up your extension's admin page and it should respond with plain text.

![](/img/guides/hellocontroller.jpeg)

Alright, so with that out of the way, let's actually render our view.

## Rendering the admin view

After the `namespace` definition, import the following classes into `controller.php`. These will be used by our functions later on in the guide.

```php [controller.php]
// Import classes needed for view rendering.
use Illuminate\View\View;
use Illuminate\View\Factory as ViewFactory;

// Import BlueprintExtensionLibrary. This is required for admin views to function.
use Pterodactyl\BlueprintFramework\Libraries\ExtensionLibrary\Admin\BlueprintAdminLibrary as BlueprintExtensionLibrary;
```

Inside of the controller's `class`, add a `__construct()` method. The `__construct()` method is automatically called on class instantiation.

```php [controller.php]
class {identifier}ExtensionController extends Controller {
  public function __construct(
    private ViewFactory $view,
    private BlueprintExtensionLibrary $blueprint,
  ) {}

  // ..your other functions
}
```

### Update the `index()` function

Because we're going to be rendering a view, we should expect our response to be one too. `View` is imported from `use Illuminate\View\View;` in [this step](#rendering-the-admin-view).

```diff [controller.php]
- public function index() {
+ public function index(): View {
```

Finally, we can return a view, instead of a plaintext response.

This code makes a view with `$this->view->make` (through ViewFactory) and makes the variables `$root` and `$blueprint` available to it.

```diff [controller.php]
public function index(): View {
- return "hello from {identifier}'s custom controller"
+ return $this->view->make(
+   'admin.extensions.{identifier}.index', [
+     'root' => "/admin/extensions/{identifier}",
+     'blueprint' => $this->blueprint,
+   ]
+ );
}
```

Save your changes and visit your extension's admin page again. You should see your admin view instead of the plaintext response from earlier.

## Adding variables to the view

Through ViewFactory's `make()` function, we can define variables that we can then use in our admin view.

The following piece of code adds a `$foo` variable to the view, with the contents of `bar`.

```diff [controller.php]
public function index(): View {
+ $foo = 'bar'
  return $this->view->make(
    'admin.extensions.{identifier}.index', [
      'root' => "/admin/extensions/{identifier}",
      'blueprint' => $this->blueprint,
+     'foo' => $foo,
    ]
  );
}
```

Then in your admin view (`admin.view` in your [conf.yml](/docs/configs/confyml#adminview-required) configuration), read out the `$foo` variable.

We're assuming your `admin.view` file is called `view.blade.php`, but it can be different depending on your extension configuration.

<!-- prettier-ignore -->
```html [view.blade.php]
<p>
  {{ $foo }}
</p>
```

Save your changes, install your extension and check out your admin view. There should be a paragraph element with 'bar' (the value of `$foo`) as it's content.

## Final results

Below is the final version of the `controller.php` file composed in the previous steps.

While there's nothing stopping you from copy pasting them into your extension, we still highly recommend going through the steps above. This file is solely here for comparison.

```php [controller.php]
<?php

// Define namespace. This is the namespace needed for admin controllers.
// {identifier} automatically gets replaced with your extension's
// identifier upon extension installation.
namespace Pterodactyl\Http\Controllers\Admin\Extensions\{identifier};

// Import classes needed for view rendering.
use Illuminate\View\View;
use Illuminate\View\Factory as ViewFactory;
// Import the controller class.
use Pterodactyl\Http\Controllers\Controller;
// Import BlueprintExtensionLibrary. This is required for admin views
// to function.
use Pterodactyl\BlueprintFramework\Libraries\ExtensionLibrary\Admin\BlueprintAdminLibrary as BlueprintExtensionLibrary;

// Register the extension-specific controller class.
class {identifier}ExtensionController extends Controller {
  public function __construct(private ViewFactory $view, private BlueprintExtensionLibrary $blueprint) {}

  // Render page. The index() function gets called when the
  // /admin/extensions/{identifier} path receives a GET request.
  public function index(): View {
    // Define the $foo variable to 'bar'.
    $foo = 'bar';
    // Render the admin view, assign $root to the URL path,
    // $blueprint to BlueprintExtensionLibrary and $foo to the $foo
    // variable. These variables can be directly used within the view.
    return $this->view->make(
      'admin.extensions.{identifier}.index', [
        'root' => "/admin/extensions/{identifier}",
        'blueprint' => $this->blueprint,
        'foo' => $foo,
      ]
    );
  }
}
```
