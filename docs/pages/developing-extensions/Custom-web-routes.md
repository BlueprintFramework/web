<div class="position-relative p-4 text-body bg-body border rounded-4 d-flex align-items-center">
  <div class="me-3">
    <i class="bi bi-book h2"></i>
  </div>
  <p class="me-3 my-0">
    Written by those who've walked the path. Want to improve our guides? Contribute and help build something awesome!
  </p>
  <a href="https://github.com/BlueprintFramework/web/tree/main/docs/pages/developing-extensions">
    <button class="btn btn-primary px-4 rounded-pill placeholder-wave" type="button">
      Contribute
    </button>
  </a>
</div><br>

# Creating custom web routes
<h4 class="fw-light">Add your own web routes accessible from everywhere within blueprint.</h4><br/>

This guide provides a step-by-step overview on how to define custom web routes using a dedicated controller within a Blueprint extension.<br/><br/>

### **Define the controller directory and routing file**

Begin by creating a new directory within your extension to store your controller classes. In this example, we will use a directory named `controllers`.

Next, open your `conf.yml` file and define the location of your controller directory and route file under the `requests` section. This allows Blueprint to recognize and load your custom logic.

```yml
requests:
  views: ""
  app: "controllers"
  routers:
    application: ""
    client: ""
    web: "routes.php"
```

- `app`: Points to the folder containing your controller classes.
- `web`: Defines the file responsible for registering your web routes.

<div class="p-2 border-start border-4 mb-5">
    <i class="bi bi-journal-text text-primary me-1"></i>
    For more details on configuration, refer to the <a href="?page=documentation/confyml">conf.yml documentation</a>.
</div>

### **Creating a controller**

Inside the `controllers` directory, create a PHP file named after your controller class. The file and class name **must match exactly**.

In this example, we will create a controller that handles routes for the dashboard and name the file `ExtensionDashboardController.php`.

```php
<?php

namespace Pterodactyl\BlueprintFramework\Extensions\{identifier};

use Pterodactyl\Http\Controllers\Controller;
use Pterodactyl\BlueprintFramework\Libraries\ExtensionLibrary\Admin\BlueprintAdminLibrary as BlueprintExtensionLibrary;

class ExtensionDashboardController extends Controller
{
    public function __construct(
        private BlueprintExtensionLibrary $blueprint,
    ) {}

    public function getData()
    {
        return response()->json([
            'status' => 'success',
        ]);
    }
}
```
- This controller defines a `getData()` method that returns a JSON response.

<div class="p-2 border-start border-4 mb-5">
    <i class="bi bi-globe text-primary me-1"></i>
    For advanced controller techniques, see the <a href="https://laravel.com/docs/10.x/controllers">Laravel controller documentation</a>.
    <br/>
    <i class="bi bi-globe text-primary me-1"></i>
    For different response types (redirects, files, etc.), refer to <a href="https://laravel.com/docs/10.x/responses">Laravel HTTP responses documentation</a>.
</div>

### **Registering routes**

Create a PHP file named `routes.php` (or another name, as long as it matches your `conf.yml`) to define your routes.

```php
<?php

use Illuminate\Support\Facades\Route;
use Pterodactyl\BlueprintFramework\Extensions\{identifier}\ExtensionDashboardController;

Route::get('/data', [ExtensionDashboardController::class, 'getData']);
```

- All routes registered in this file are automatically prefixed with `/extensions/{identifier}`.
- This example registers a new GET route accessible at `/extensions/{identifier}/data` and connects it to the `getData` function from `ExtensionDashboardController`.

<div class="p-2 border-start border-4 mb-5">
    <i class="bi bi-exclamation-diamond text-warning me-1"></i>
    Be sure to update the second <code>use</code> statement to match the correct namespace and class name of your controller.
    <br/>
    <i class="bi bi-globe text-primary me-1"></i>
    For detailed route configuration, visit the <a href="https://laravel.com/docs/10.x/routing">Laravel routing documentation</a>.
</div>