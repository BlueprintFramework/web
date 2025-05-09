<div class="position-relative p-4 text-body bg-body border rounded-4 d-flex align-items-center">
  <div class="me-3">
    <i class="bi bi-book h2"></i>
  </div>
  <p class="me-3 my-0">
    Written by those who've walked the path. Want to improve our guides? Contribute and help build something awesome!
  </p>
  <a href="https://github.com/BlueprintFramework/web/edit/main/docs/pages/developing-extensions/Dashboard-wrappers.md">
    <button class="btn btn-primary px-4 rounded-pill placeholder-wave" type="button">
      Contribute
    </button>
  </a>
</div><br>

# Dashboard wrappers
<h4 class="fw-light">Extend the Pterodactyl client and admin dashboard within the Laravel blade wrapper.</h4><br/>

Dashboard wrappers allow you to extend both the **admin** and **client** dashboard views with custom logic rendered server-side via Laravel Blade templates.
They are ideal for displaying dynamic data, utilizing PHP logic, or integrating features from the [BlueprintExtensionLibrary](?page=documentation/$blueprint).

<div class="alert mt-2 rounded-4 border" role="alert">
  <i class="bi bi-exclamation-diamond mb-1 text-warning float-start fs-4"></i>
  <div class="ps-3 ms-3">While wrappers are rendered on the server, any JavaScript inside them will still be executed on the client as usual.</div>
</div><br/>

## Configuration

To register a wrapper, define it in your extension’s `conf.yml` under the desired section:

- `admin`: Adds the wrapper to all admin pages under `/admin/`

- `dashboard`: Adds the wrapper to all non-admin panel pages (e.g., server overview, server console, etc.)

You can also target both areas by specifying the same wrapper for each.

```yml
admin:
  wrapper: "wrapper.blade.php"

dashboard:
  wrapper: "wrapper.blade.php"
```
<br/>

## Example wrapper

Below is a minimal example showing how a wrapper can be used to conditionally render content based on a database value.

```php
@php
  $isEnabled = $blueprint->dbGet('spaceinvaders', 'enabled');
@endphp

@if($isEnabled)
  <h1>Extension is enabled!</h1>
@else
  <h1>Extension is not enabled.</h1>
@endif
```

This checks the `enabled` setting of the `spaceinvaders` extension and displays a corresponding message. While this example is simple, it demonstrates how you can embed logic using the `BlueprintExtensionLibrary` within your Blade wrappers.

<div class="alert mt-2 rounded-4 border" role="alert">
  <i class="bi bi-globe mb-1 float-start fs-4"></i>
  <div class="ps-3 ms-3">For more information about blade views, visit <a href="https://laravel.com/docs/10.x/blade">Laravel’s blade template documentation</a>.</div>
</div><br/>

<div class="btn-group docs-navigator" role="group" aria-label="Navigation" style="float: right">
  <a href="?page=developing-extensions/Custom-table-and-migrations" class="btn btn-dark bg-light-subtle border-0 rounded-start-pill">Previous</a>
  <a href="?page=developing-extensions/React-components" class="btn btn-dark bg-light-subtle border-0 rounded-end-pill">Next</a>
</div>