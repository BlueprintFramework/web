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
