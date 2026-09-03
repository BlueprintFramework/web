---
title: Your second admin page
description: Add configuration options to the admin page
author: Sirzento and Emma
category: dev
thumbnail: adminconfiguration.jpeg
order: 4
---

## Introduction

This guide demonstrates how to create an admin controller that loads and saves extension-specific configuration values using the [BlueprintExtensionLibrary](/docs/lib/methods) database methods. It also covers how to define the corresponding migration to initialize default configuration values, along with input validation through a custom form request.

## Controller

### Controller setup

After creating your [custom controller](/guides/dev/admincontroller), you can now extend it to load and store configuration values.

Start by importing the necessary classes into the `admin.controller`:

```php [controller.php]
namespace Pterodactyl\Http\Controllers\Admin\Extensions\{identifier};

use Illuminate\View\View;
use Illuminate\View\Factory as ViewFactory;
use Pterodactyl\Http\Controllers\Controller;
use Pterodactyl\Http\Requests\Admin\AdminFormRequest;
use Illuminate\Http\RedirectResponse;

use Pterodactyl\BlueprintFramework\Libraries\ExtensionLibrary\Admin\BlueprintAdminLibrary as BlueprintExtensionLibrary;
```

Most of these classes should be familiar from the [custom controller](/guides/dev/admincontroller) setup.

Here’s what’s new:

- `AdminFormRequest`: Handles form input validation.
- `RedirectResponse`: Used to return the user back to the admin page after saving changes.

Inject the `SettingsRepositoryInterface` into your controller constructor:

```php [controller.php]
public function __construct(
  private ViewFactory $view,
  private BlueprintExtensionLibrary $blueprint,
  private SettingsRepositoryInterface $settings,
) {}
```

### Loading configuration

To handle configuration, extend your `index()` function to retrieve the current values.

For this example, we will add the configuration `theme`, `customName` and `count`:

```php [controller.php]
public function index(): View
{
  $theme = $this->blueprint->dbGet('{identifier}', 'theme');
  $customName = $this->blueprint->dbGet('{identifier}', 'customName');
  $count = $this->blueprint->dbGet('{identifier}', 'count');

  return $this->view->make(
    'admin.extensions.{identifier}.index', [
      'theme' => $theme,
      'customName' => $customName,
      'count' => $count,
      'root' => "/admin/extensions/{identifier}",
      'blueprint' => $this->blueprint,
  ]
);
```

The `$blueprint->dbGet()` function is used to retrieve values from the database.
At the end of the function, the Blade view is returned along with the configuration values.

::card
For more information about Blueprints database methods, take a look at the [BlueprintExtensionLibrary documentation](/docs/lib/methods).
::

### Saving configuration

To save the configuration, add an `update()` function to your controller. This will be called when the user submits the admin form located in your view via `PATCH` request.

```php [controller.php]
public function update({identifier}SettingsFormRequest $request): RedirectResponse
{
  foreach ($request->normalize() as $key => $value) {
    $this->blueprint->dbSet("{identifier}", $key, $value);
  }

  return redirect()->route('admin.extensions.{identifier}.index');
}

```

The `update()` function accepts a validated form request and saves each setting using the `dbSet()` function of the [BlueprintExtensionLibrary](/docs/lib/methods#set-a-record-in-the-database). At the end you will be redirect to the `index()` function to refresh all the data on your extension's admin page to check if everything got saved correctly.

### Input validation

Finally, create a form request class at the bottom of the controller file (outside of your controller class) to validate incoming form data:

```php [controller.php]
class {identifier}SettingsFormRequest extends AdminFormRequest
{
  public function rules(): array
  {
    return [
      'theme' => ['string', 'in:1,2,3'],
      'customName' => ['string'],
      'count' => ['numeric'],
    ];
  }

  public function attributes(): array
  {
    return [
      'theme' => 'Theme',
      'customName' => 'Custom Name',
      'count' => 'Count',
    ];
  }
}

```

- `rules()` defines the validation rules for each input field.
- `attributes()` provides user-friendly names for use in validation error messages.

::card
For more information about validation rules, see [Laravel’s validation documentation](https://laravel.com/docs/10.x/validation#available-validation-rules).
::

## Defining default configuration values with migrations

::card
This section provides a basic introduction to migrations. For a more in-depth understanding, as well as how to actual define migrations in Blueprint, refer to the [Custom table and migrations guide](/guides/dev/migrations) before continuing.
::

To define the structure of your extension's configuration and register default values, you can use a database migration. This ensures your settings are initialized correctly when the extension is installed.

[Create a migration file](/guides/dev/migrations) and include the following logic:

```php
<?php

use Illuminate\Support\Facades\DB;
use Illuminate\Database\Migrations\Migration;
use Pterodactyl\BlueprintFramework\Libraries\ExtensionLibrary\Admin\BlueprintAdminLibrary as BlueprintExtensionLibrary;

return new class extends Migration
{
  public function up(): void
  {
    $blueprint = app(BlueprintExtensionLibrary::class);

    $blueprint->dbSetMany("{identifier}", [
      'theme' => '0',
      'customName' => 'superuser',
      'count' => 10,
    ]);
  }

  public function down(): void
  {
    DB::table('settings')->where('key', 'like', '{identifier}::%')->delete();
  }
};
```

- The `up` function inserts default key-value pairs via [BlueprintExtensionLibrary's `dbSetMany()` function](/docs/lib/methods#set-multiple-records-in-the-database).
- The `down` function reverses these changes by removing all settings entries that match your extension’s key prefix.

## View

### Form structure

```php
<form id="config-form" action="" method="POST">
  ...
  {{ csrf_field() }}
  <button type="submit" name="_method" value="PATCH" class="btn btn-primary">Save Changes</button>
</form>
```

This is the basic form structure you can use in your view. Don't be surprised that the `POST` method is used in the form and not `PATCH`.
This is because [HTML Forms do not support this request method](https://laravel.com/docs/10.x/routing#form-method-spoofing). So that we can still send the message as `PATCH`, meta information is added to the submit button. Here you can see that the request type is set to `PATCH` via `name="_method"` and `value="PATCH"`.

`{{ csrf_field() }}` inserts a hidden CSRF token input into the form, which Laravel uses to protect against Cross-Site Request Forgery (CSRF) attacks. It ensures that the form submission is coming from an authenticated and trusted source.

### Form fields

To go along with the current example, we add a text `input` for `customName`, a `dropdown` for `theme` and a number `input` for `count` to the `form`:

```html [view.blade.php]
<form id="config-form" action="" method="POST">
  <input
    type="text"
    name="customName"
    id="customName"
    value="{{ $customName }}"
    placeholder="type here"
    class="form-control"
  />
  <select class="form-control" name="theme">
    <option value="0" @if($theme="" ="0" ) selected @endif>Theme 1</option>
    <option value="1" @if($theme="" ="1" ) selected @endif>Theme 2</option>
    <option value="2" @if($theme="" ="2" ) selected @endif>Theme 3</option>
  </select>

  <input
    type="number"
    name="count"
    id="count"
    value="{{ $count }}"
    placeholder="10"
    class="form-control"
  />

  {{ csrf_field() }}
  <button type="submit" name="_method" value="PATCH" class="btn btn-primary">
    Save Changes
  </button>
</form>
```

And now you're done. You should be able to load and save your admin configuration!
