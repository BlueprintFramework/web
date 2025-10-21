---
title: Console.yml
description: Define and schedule Artisan commands
category: configs
---

::card
The `Console.yml` configuration lives in the root of an extension's `data.console` directory. You have to create a directory, bind it in [conf.yml](/docs/configs/confyml#dataconsole) and create a `Console.yml` file. All Artisan commands should be placed in this (or a sub-)directory too.
::

## Introduction

`Console.yml` allows your extension to register Artisan commands (PHP scripts), and optionally schedule them on an interval. Artisan commands can be called by system administrators, scripts, controllers and schedules.

## Creating a simple command

Define your command in your Console.yml configuration:

<!-- prettier-ignore -->
```yaml [Console.yml]
# Sets Signature to 'foo', Description to 'returns bar' and binds Path to 'foo.php'
- { Signature: 'foo', Description: 'Returns bar', Path: 'foo.php', Interval: '' }
```

Create the `foo.php` file (defined in the `Path` bind), relative to the `data.console` directory.

```php [foo.php]
<?php

echo('bar');
```

After installing/building your extension, run `php artisan myextension:foo` in your Pterodactyl webserver directory and see it in action!

## Scheduling

Artisan commands can be scheduled. Intervals should be put in the `Interval` option in the Console.yml configuration. We support both cron and human-readable intervals.

### Human-readable intervals

| Interval              | Description                        |
| --------------------- | ---------------------------------- |
| `everyMinute`         | Run the task every minute          |
| `everyTwoMinutes`     | Run the task every two minutes     |
| `everyThreeMinutes`   | Run the task every three minutes   |
| `everyFourMinutes`    | Run the task every four minutes    |
| `everyFiveMinutes`    | Run the task every five minutes    |
| `everyTenMinutes`     | Run the task every ten minutes     |
| `everyFifteenMinutes` | Run the task every fifteen minutes |
| `everyThirtyMinutes`  | Run the task every thirty minutes  |
| `hourly`              | Run the task every hour            |
| `daily`               | Run the task every day             |
| `weekdays`            | Run the task every weekday         |
| `weekends`            | Run the task every weekend-day     |
| `sundays`             | Run the task every sunday          |
| `mondays`             | Run the task every monday          |
| `tuesdays`            | Run the task every tuesday         |
| `wednesdays`          | Run the task every wednesday       |
| `thursdays`           | Run the task every thursday        |
| `fridays`             | Run the task every friday          |
| `saturdays`           | Run the task every saturday        |
| `weekly`              | Run the task every week            |
| `monthly`             | Run the task every month           |
| `quarterly`           | Run the task every quarter         |
| `yearly`              | Run the task every year            |

### Cron-syntax intervals

Cron-syntax can be used instead of human-readable intervals if more-specific intervals are desired. More information about cron scheduling can be found on [it's Wikipedia page](https://en.wikipedia.org/wiki/Cron).

## Differences from standard Artisan commands

Unlike standard Artisan commands, Blueprint bootstraps your command and runs your PHP script. Here's an example of what should leave out:

::card
This is a simplified example of [Blueprint's built-in version checker](https://github.com/BlueprintFramework/framework/blob/main/app/Console/Commands/BlueprintFramework/Version/VersionLatestCommand.php). We've ommitted/adjusted some irrelevant parts from the file shown below.
::

```diff [is_latest.php]
<?php
- namespace Pterodactyl\Console\Commands\Path\To\Namespace;
-
- use Illuminate\Console\Command;
- use Pterodactyl\BlueprintFramework\Libraries\ExtensionLibrary\Console\BlueprintConsoleLibrary as BlueprintExtensionLibrary;
-
- class VersionLatestCommand extends Command {
-   protected $signature = 'myextension:islatest';
-   protected $description = 'Check if the installed Blueprint version is the latest version';
-
-   public function __construct(private BlueprintExtensionLibrary $blueprint) {
-     parent::__construct();
-   }
-
-   public function handle() {
      // This command checks if Blueprint's current version is the latest version
      // using Blueprint's version cache.
      $latest = $this->blueprint->dbGet('blueprint', 'internal:version:latest');
      if ($latest == '') {
        $this->call('bp:version:cache');
        $latest = $this->blueprint->dbGet('blueprint', 'internal:version:latest');
      }

      echo $latest;
      return $latest;
-   }
- }
```

As you can see, we aren't defining a `namespace`, `class`, `signature`, `description`, etc. For a Blueprint extension, these are simply not needed and directly handled by Blueprint.

The `signature` and `description` is defined in your Console.yml. Keep in mind that Blueprint prefixes your extension's signature with it's identifier to avoid conflicts. (`mycommand` -> `myextension:mycommand`)

## Configuration

### Example configuration

Here is an example (valid) Console.yml configuration.

<!-- prettier-ignore -->
```yaml [Console.yml]
- { Signature: "foo", Description: "returns bar", Path: "foobar.php", Interval: "" }
- { Signature: "byte", Description: "[  ^ ^]", Path: "byte.php", Interval: "everyMinute" }
- { Signature: "hello", Description: "prints 'hello world'", Path: "folder/hello.php", Interval: "*/5 * * * *" }
```

### Reference

As a point of reference, below is a configuration file with all supported configuration options for Console.yml.

<!-- prettier-ignore -->
```yaml [Console.yml]
- { Signature: '', Description: '', Path: '', Interval: '' }
```
