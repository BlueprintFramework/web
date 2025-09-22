---
title: Methods
description: BlueprintExtensionLibrary functions and examples
category: lib
---

## Alert

You can use `alert()` to display an alert message at the top of an admin page. This method is only available in admin views and controllers.

| Param      | Type                        | Description   |
| ---------- | --------------------------- | ------------- |
| `$type`    | info/warning/danger/success | Alert type    |
| `$message` | string                      | Alert content |

```php
$blueprint->alert('info', 'The quick brown fox jumps over the lazy dog');
$blueprint->alert('warning', 'The lazy dog woke up');
$blueprint->alert('danger', 'The lazy dog is chasing the quick brown fox');
$blueprint->alert('success', 'The quick brown fox got away');
```

The alert method returns `void`.

## Database

Blueprint's database methods allow extensions to interact with the panel's database within views, console commands and controllers.

### Get a record from the database

The `dbGet()` method allows you to get a record from the database. It expects `$table` and `$record` parameters and fetches a singular record.

| Param                 | Type   | Description                                         |
| --------------------- | ------ | --------------------------------------------------- |
| `$table`              | string | Database table, usually your extension's identifier |
| `$record`             | string | Database record                                     |
| `$default` (optional) | mixed  | Returns this value when value is null               |

```php
// Get a record from the database
$foo = $blueprint->dbGet('{identifier}', 'foo');

// Print the record's value
echo($foo);
```

### Get multiple records from the database

The `dbGetMany()` method retrieves multiple records from the database. Data is automatically unserialized.

| Param                 | Type   | Description                                                 |
| --------------------- | ------ | ----------------------------------------------------------- |
| `$table`              | string | Database table, usually your extension's identifier         |
| `$records` (optional) | array  | Array of record names. If empty, gets all records for table |
| `$default` (optional) | mixed  | Returns this value when value is null                       |

```php
// Get specific records
$data = $blueprint->dbGetMany('{identifier}', ['foo', 'bar', 'baz']);

// Get all records for a table
$allData = $blueprint->dbGetMany('{identifier}');
```

Returns an associative array with record names as keys.

### Set a record in the database

The `dbSet()` method stores a value in the database. Data is automatically serialized.

| Param     | Type   | Description                                         |
| --------- | ------ | --------------------------------------------------- |
| `$table`  | string | Database table, usually your extension's identifier |
| `$record` | string | Database record                                     |
| `$value`  | mixed  | Value to store                                      |

```php
$blueprint->dbSet('{identifier}', 'foo', 'some value');
$blueprint->dbSet('{identifier}', 'settings', ['enabled' => true, 'count' => 42]);
```

### Set multiple records in the database

The `dbSetMany()` method stores multiple values at once. More efficient than multiple `dbSet()` calls.

| Param      | Type   | Description                                         |
| ---------- | ------ | --------------------------------------------------- |
| `$table`   | string | Database table, usually your extension's identifier |
| `$records` | array  | Associative array of record => value pairs          |

```php
$blueprint->dbSetMany('{identifier}', [
    'foo' => 'value1',
    'bar' => 'value2',
    'settings' => ['enabled' => true]
]);
```

### Delete a record from the database

The `dbForget()` method removes a single record from the database.

| Param     | Type   | Description                                         |
| --------- | ------ | --------------------------------------------------- |
| `$table`  | string | Database table, usually your extension's identifier |
| `$record` | string | Database record to delete                           |

```php
$deleted = $blueprint->dbForget('{identifier}', 'foo');
// Returns true if a record was deleted, false if it didn't exist

if($deleted) {
  echo("database records have been deleted");
} else {
  echo("something went wrong :c")
}
```

### Delete multiple records from the database

The `dbForgetMany()` method removes multiple records at once.

| Param      | Type   | Description                                         |
| ---------- | ------ | --------------------------------------------------- |
| `$table`   | string | Database table, usually your extension's identifier |
| `$records` | array  | Array of record names to delete                     |

```php
$deleted = $blueprint->dbForgetMany('{identifier}', ['foo', 'bar', 'baz']);
```

### Delete all records from a table

The `dbForgetAll()` method removes all records for a given table.

| Param    | Type   | Description                                         |
| -------- | ------ | --------------------------------------------------- |
| `$table` | string | Database table, usually your extension's identifier |

```php
$deleted = $blueprint->dbForgetAll('{identifier}');
```

## Extensions

Extension methods allow you to check if an extension is installed, query an extension's configuration and more.

### Check if an extension is installed

The `extension()` method checks if a specific extension is installed by its identifier.

| Param         | Type   | Description          |
| ------------- | ------ | -------------------- |
| `$identifier` | string | Extension identifier |

```php
$isInstalled = $blueprint->extension('myextension');
// Returns true if installed, false otherwise

if($isInstalled) {
  echo("myextension is installed!");
} else {
  echo("myextension isn't installed :(");
}
```

### Get all installed extensions

The `extensions()` method returns an array of all installed extension identifiers.

```php
$installedExtensions = $blueprint->extensions();
// Returns array like ['myextension', 'minecraftplayermanager', 'nebula']
```

### Get extension configuration

The `extensionConfig()` method retrieves the configuration for a specific extension.

| Param         | Type   | Description          |
| ------------- | ------ | -------------------- |
| `$identifier` | string | Extension identifier |

```php
$config = $blueprint->extensionConfig('myextension');
// Returns array with extension config, or null if extension doesn't exist
```

### Get all extension configurations

The `extensionsConfigs()` method returns a [Laravel Collection](https://laravel.com/docs/10.x/collections) containing configurations for all installed extensions.

```php
$allConfigs = $blueprint->extensionsConfigs();
// Returns Collection of config arrays
```

## Import

Import methods are **only available** in admin/client views and controllers. They allow your extension to add external stylesheets and scripts to views without worrying about browser cache.

### Import stylesheet

The `importStylesheet()` method generates an HTML link tag for stylesheets with cache-busting parameters. Only available in admin views.

| Param  | Type   | Description    |
| ------ | ------ | -------------- |
| `$url` | string | Stylesheet URL |

```php
echo $blueprint->importStylesheet('{webroot/public}/style.css');
// Outputs: <link rel="stylesheet" href="{webroot/public}/style.css?v=123456">
```

### Import script

The `importScript()` method generates an HTML script tag with cache-busting parameters. Only available in admin views.

| Param  | Type   | Description |
| ------ | ------ | ----------- |
| `$url` | string | Script URL  |

```php
echo $blueprint->importScript('{webroot/public}/script.js');
// Outputs: <script src="{webroot/public}/script.js?v=123456"></script>
```
