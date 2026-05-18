---
title: Custom table and migrations
description: Add custom database tables to store user-specific or complex data structures.
author: Sirzento and Emma
category: dev
thumbnail: migrations.jpeg
order: 5
---

While the [BlueprintExtensionLibrary](/docs/lib/methods#database)'s `dbGet()` and `dbSet()` functions provide a convenient way to store extension-wide data, sometimes more granular control is needed—such as storing user-specific settings or complex datasets. This is where custom database tables and migrations come into play.

## Defining migrations

To begin, define what kind of data your extension should store and create a new migration to handle the database structure. Migrations are used to extend the panel's database schema with extension-specific tables.

First, specify a directory to store your migration files in the [`database.migrations` conf.yml bind](/docs/configs/confyml#databasemigrations):

```yml [conf.yml]
database:
  migrations: 'migrations'
```

::card
You define a directory, **not a single file**, because migrations are versioned and only run once. Future updates to your schema should be handled by additional migration files.
::

### Migration file structure

Migration files must follow a strict naming convention:
`YYYY_MM_DD_HHMMSS_migration_name.php`.

This ensures they are executed in the correct order. Use the current date and time when naming your file.

As example, we create a table for custom user specific data:

```php [2025_04_23_163000_add_userdata_table.php]
<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
  /**
    * Run the migrations.
    */
  public function up(): void
  {
    Schema::create('{identifier}_userdata', function (Blueprint $table) {
      $table->unsignedInteger('user_id');
      $table->foreign('user_id')->references('id')->on('users');
      $table->boolean('enabled')->default(0);
      $table->string('customName')->default("");
      $table->json('categories')->default("[]");
    });
  }

  /**
    * Reverse the migrations.
    */
  public function down(): void
  {
    Schema::drop('{identifier}_userdata');
  }
};
```

Each migration file contains two primary functions: `up()` and `down()`. These define what changes are made to the database when the migration is run, and how to undo them.

- The `up()` function is executed when the migration is first applied. It defines the structural changes you want to make to the database.
  In the example above, it creates a new table named `{identifier}_userdata`. This table includes:
  - A foreign key `user_id` referencing the `users` table,
  - A boolean field `enabled` (default: `0`),
  - A string field `customName` (default: empty string),
  - A JSON field `categories` (default: empty array `[]`).

- The `down()` function is the reverse of `up()`. It defines how to **rollback** the migration if needed.
  In most cases, such as this example, you simply remove the table using `Schema::dropIfExists('{identifier}_userdata')`.

This structure makes database migrations both forward-compatible (via `up()`) and reversible (via `down()`), providing a clean way to version and maintain schema changes over time.

::card
For more information about migrations, take a look inside [Laravel's migration documentation](https://laravel.com/docs/10.x/migrations).
::

## Reading data from your table

To retrieve data, use Laravel’s built-in `DB` facade. First, import it in your controller:

```php
use Illuminate\Support\Facades\DB;
```

Then, define a function to query your table. In this function we want to get the `categories` of the current user:

```php
public function getCategories() {
  // Check if the user of the current request exists
  $user = auth()->user();
  if ($user == null) {return response(null);}

  // Gets the data corrensponding to the user id
  $data = DB::table('{identifier}_userdata')->where('user_id', $user->id)->first();
  // Check if data exists
  if ($data == null) {return response(null);}

  // Returns the requested data
  return response($data->categories);
}
```

This function checks for an authenticated user, queries the table using their ID, and returns the `categories` field if found.

## Saving data to the table

Saving works similarly as the logic used at the [admin configuration guide](/guides/dev/adminconfiguration).

```php
public function update({identifier}UserSettingsFormRequest $request) {
  $userId = auth()->user()->id;
  $valuesToUpdate = $request->normalize();

  DB::table('{identifier}_userdata')
    ->updateOrInsert(
      ['user_id' => $userId],
      $valuesToUpdate
    );

  return response()->json($valuesToUpdate);
}
```

This function saves or updates the user’s row in your custom table. The `updateOrInsert()` function handles both creation and updating automatically.

## Input validation

Define a custom form request at the end of your controller file to validate incoming data:

```php
class {identifier}UserSettingsFormRequest extends AdminFormRequest
{
  public function rules(): array
  {
    return [
      'enabled' => 'nullable|numeric|min:0|max:1',
      'customName' => 'nullable|string',
      'categories' => 'nullable|string',
    ];
  }

  public function attributes(): array
  {
    return [
      'enabled' => 'Enabled',
      'categories' => 'Categorie',
      'customName' => 'Custom Name',
    ];
  }
}
```

This ensures incoming data matches the expected format and helps prevent invalid writes to your table.

With this setup, your extension can now store complex or user-scoped data efficiently.
