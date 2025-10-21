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

## Creating a command

Start by defining your command in your Console.yml configuration.

<!-- prettier-ignore -->
```yaml [Console.yml]
# Sets Signature to 'foo', Description to 'returns bar' and binds Path to 'foo.php'
- { Signature: 'foo', Description: 'Returns bar', Path: 'foo.php', Interval: '' }
```

```yaml [Console.yml]
- { Signature: '', Description: '', Path: '', Interval: '' }
```

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
