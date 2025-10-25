---
title: Custom routes
description: Add custom endpoints to your extension
category: concepts
---

## Introduction

Blueprint allows extensions to create custom routing configurations using [Laravel's route definitions](https://laravel.com/docs/10.x/routing). These routes can extend the Pterodactyl API, but can also add generic routes. Custom routes utilize the `requests.routers` [conf.yml bind](/docs/configs/confyml#requestsrouters).

### Router types

Extensions can bind 3 router types: `requests.routers.application`, `requests.routers.client` and `requests.routers.web`. Each have their own purpose and authentication requirements.

| Bind                           | URL prefix                                 | Authentication                                                       |
| ------------------------------ | ------------------------------------------ | -------------------------------------------------------------------- |
| `requests.routers.application` | `/api/application/extensions/{identifier}` | Application API key (generated in the admin panel)                   |
| `requests.routers.client`      | `/api/client/extensions/{identifier}`      | Client API key (generated in user account settings) or session token |
| `requests.routers.web`         | `/extensions/{identifier}`                 | No authentication                                                    |

As you can see above, Blueprint adds a URL prefix to each route to prevent conflicts between extensions. This is handled automatically. Your route definition should be interpreted as relative to the URL prefix.

## Basic routing

A route is basically a definition of what to do when a request comes in matching the exact path and HTTP method.

Create a `web.php` file and bind it to `requests.routers.web` in your [conf.yml](/docs/configs/confyml#requestsroutersweb).

```yaml [conf.yml]
requests:
  routers:
    # bind your custom web routes definition
    # to the requests.routers.web config option :)
    web: 'web.php'
```

After binding it, open up your `web.php` file and add the code below. This route will return "bar" to the request whenever it is triggered.

```php [web.php]
<?php

// Import Laravel's route facade
use Illuminate\Support\Facades\Route;

// Trigger this route only on a GET request to it
//      |   Create a '/foo' route (which becomes '/extensions/{identifier}/foo')
//      |     |
Route::get('/foo', function () {
  return 'bar';
});
```

Install your extension and visit `/extensions/{identifier}/foo` in your URL bar. You should see it respond with "bar".

## Using controllers

Instead of handling all requests in your route files, you can send requests over to controllers.

If you don't have an `requests.app` bind yet, create a directory and set the [conf.yml bind](/docs/configs/confyml#requestsapp) to it.

```yaml [conf.yml]
requests:
  app: 'app' # << the directory for your controllers to live in
  routers:
    web: 'web.php'
    # ^^ the custom web router you created earlier
```

In your `requests.app` directory (which is called `app` in this case), create a new file called `FooController.php` with the following content:

```php [app/FooController.php]
<?php

// This is the namespace 'requests.app' lives in. {appcontext}
// is automatically corrected by Blueprint upon installing your
// extension.
namespace {appcontext};

use Pterodactyl\Http\Controllers\Controller;

// This is the class you'll be referencing in your router.
class FooController extends Controller {
  // This is the function of FooController your router will
  // call whenever the route is called.
  public function index() {
    return 'bar (but using a controller)';
  }
}
```

Finally, update your `requests.routers.web` router. Import the `requests.app` namespace and bind `/foo` to `FooController`'s `index()` class.

```diff [web.php]
<?php

use Illuminate\Support\Facades\Route;
+ use {appcontext};

- Route::get('/foo', function () {
-   return 'bar';
- });
+ Route::get('/foo', [FooController::class, 'index']);
```

## Rendering a view

Last but not least, we can go full-circle by utilizing the `requests.views` [conf.yml bind](/docs/configs/confyml#requestsviews) to actually render web pages!

::card
Laravel views should not be used for extending the user-side dashboard! If you are looking to properly extend that side of Pterodactyl, you should look into [Components.yml](/docs/configs/componentsyml) instead.

You should definitely, however, use the [controller-method documented above](#using-controllers) for creating API routes that can be called by the frontend API. The how-to is pretty much the same no matter the controller type, though [you can check the differences here](#router-types).
::

Create a `views` directory (if you don't have one already) and assign it to the `requests.views` bind.

```yaml [conf.yml]
requests:
  # where your views live in
  views: 'views'
  # and then all the stuff you added earlier
  app: 'app'
  routers:
    web: 'web.php'
```

With your `requests.views` directory created, create a `fizz.blade.php` file inside of it. This is your Laravel Blade view, which will be rendered by your controller.

::card
Laravel's Blade views can be quite useful and versitile. You can read more about [Blade templates and it's directives in the Laravel documentation](https://laravel.com/docs/10.x/blade#blade-directives).
::

<!-- prettier-ignore -->
```html [views/fizz.blade.php]
<p style="background: black; color: white;">
  buzz!
</p>
```

Create a controller called `FizzController.php` and make it render the `fizz.blade.php` view.

```php [app/FizzController.php]
<?php

namespace {appcontext};

use Pterodactyl\Http\Controllers\Controller;

// Import the View and ViewFactory classes for view rendering.
use Illuminate\View\View;
use Illuminate\View\Factory as ViewFactory;

class FizzController extends Controller {
  public function __construct(private ViewFactory $view) {}

  // Create another index function, this time promising a view
  // to be returned.
  public function index(): View {
    // Render a view. Views should be prefixed with {viewcontext}
    // which is automatically replaced by Blueprint.
    return $this->view->make('{viewcontext}.fizz');
  }
}
```

Add the `/fizz` route to your `requests.routers.web` router.

```diff [web.php]
<?php

use Illuminate\Support\Facades\Route;
use {appcontext};

Route::get('/foo', [FooController::class, 'index']);
+ Route::get('/fizz', [FizzController::class, 'index']);
```

Install your extension and visit the `/extensions/{identifier}/fizz` path. You should see a text saying "buzz!" with a black background.
