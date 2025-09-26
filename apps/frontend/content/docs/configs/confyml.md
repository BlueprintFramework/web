---
title: conf.yml
description: Configure your extension and define your directory structure
category: configs
---

## Introduction

Every extension needs a `conf.yml` config file at its heart - this is where you define metadata, set flags, and bind files/directories to Blueprint's extension features. Blueprint parses this file during installation and validates all the specified paths before proceeding with the installation process.

## Configuration options

### Info section

The info section contains all the metadata about your extension that gets displayed to administrators and used internally by Blueprint for tracking and validation.

#### `info.name` (required)

The extension's display name, shown primarily in the admin panel.

```yaml [conf.yml]
info:
  # the name of your extension, write
  # something creative! you can always change
  # it later
  name: 'My extension'
```

#### `info.identifier` (required)

Differentiates your extension from others - used for folder trees, routers, file names, etc. Extension identifiers must be unique from other extensions and can only contain `a-z` characters.

```yaml [conf.yml]
info:
  # the unique identifier of your extension,
  # you can't really (easily) change this later
  identifier: 'myextension'
```

#### `info.description` (required)

Extension description shown on your extension's admin page.

```yaml [conf.yml]
info:
  # shown to administrators on the extension's
  # admin page
  description: 'This is an awesome extension!'
```

#### `info.flags`

A list of comma-separated extension flags.

::card
Flags enable (or sometimes disable) advanced and experimental Blueprint features. Learn more about them in the [Flags article](/docs/concepts/flags). All flags unknown to Blueprint are ignored.
::

```yaml [conf.yml]
info:
  # advanced and experimental flags. don't know
  # what these are? you probably don't need them
  flags: 'flag1, flag2, flag3'
```

#### `info.version` (required)

Extension's current version.

```yaml [conf.yml]
info:
  # the extension's version
  version: '1.0'
```

#### `info.target` (required)

Blueprint version your extension is designed for. Blueprint compares this against the current version during installation and warns users if versions don't match, but still allows installation.

```yaml [conf.yml]
info:
  # when this doesn't match a panel's blueprint
  # version, they may see a warning about it
  target: 'beta-2025-09'
```

#### `info.author`

Your (development team's) name.

```yaml [conf.yml]
info:
  # put your own name here
  author: 'Nothing To Be Seen Here Industries'
```

#### `info.icon`

Extension icon shown in the admin panel. Supports `.jpg`/`.jpeg`, `.svg`, `.png`, `.gif`, `.webp`. Blueprint uses a random placeholder icon if this is left blank.

```yaml [conf.yml]
info:
  # icons can be jpg/jpeg, svg, png, gif or webp!!
  icon: 'path/to/icon.png'
```

#### `info.website`

Website linked in the top-right of your extension's admin page.

```yaml [conf.yml]
info:
  # some urls have different icons on the extension's
  # admin page :)
  website: 'https://prpl.wtf'
```

### Admin section

Controls how your extension integrates with Pterodactyl's admin panel. Blueprint creates admin routes and controllers based on these settings.

#### `admin.view` (required)

Path to your admin blade view. Blueprint copies this to `resources/views/admin/extensions/{identifier}/` and creates the appropriate route structure. This is the main interface administrators will see.

```yaml [conf.yml]
admin:
  # blade views are basically html with support for
  # php logic!
  view: 'path/to/view.blade.php'
```

#### `admin.controller`

Path to custom admin view controller. If not defined, Blueprint uses its built-in controller which provides basic functionality. Gets copied to `app/Http/Controllers/Admin/Extensions/{identifier}/` if specified.

```yaml [conf.yml]
admin:
  # when kept empty, blueprint will use a basic
  # built-in controller
  controller: 'path/to/controller.php'
```

#### `admin.css`

Custom css for Pterodactyl admin panel.

```yaml [conf.yml]
admin:
  # plain css added into the admin panel
  css: 'path/to/admin.css'
```

#### `admin.wrapper`

Extends admin panel layout with additional blade view content. Gets copied to `resources/views/blueprint/admin/wrappers/{identifier}.blade.php` and included in the admin layout.

```yaml [conf.yml]
admin:
  # some additional html/php added to the end of
  # the admin panel
  wrapper: 'path/to/wrapper.blade.php'
```

### Dashboard section

controls integration with pterodactyl's client-facing dashboard. works similarly to admin section but for end users.

#### `dashboard.css`

Custom css for Pterodactyl client panel. Is imported into the React application.

::card
Unlike `admin.css`, `dashboard.css` is not imported from the wrapper, but added to the React bundle instead. In other words, dashboard css might behave slightly differently. Syntax errors may temporary prevent the client-side dashboard from rendering properly until resolved. Alternatively, you can add stylesheets between `<style/>` tags in your `dashboard.wrapper` to mimic `admin.css` behavior.
::

```yaml [conf.yml]
dashboard:
  # css added to the pterodactyl dashboard
  css: 'path/to/dashboard.css'
```

#### `dashboard.wrapper`

Extends client dashboard layout with additional blade view content. Copied to `resources/views/blueprint/dashboard/wrappers/{identifier}.blade.php` and included in the dashboard wrapper.

::card
The `dashboard.wrapper` allows extensions to add PHP logic and additional HTML into the client dashboard view. Interacting with the dashboard's components through this requires complicated workarounds and is advised against, use `dashboard.components` for that instead.
::

```yaml [conf.yml]
dashboard:
  # html/php added to the end of the frontend page,
  # outside of the react bundle
  wrapper: 'path/to/wrapper.blade.php'
```

#### `dashboard.components`

React components directory for extending the Pterodactyl user-side frontend.

::card
This binding is related to [Components.yml](/docs/configs/componentsyml). Learn more about what it does, why it exists and how to use it in that article.
::

```yaml [conf.yml]
dashboard:
  # directory for components (Components.yml)
  components: 'path/to/directory'
```

### Data section

Defines where Blueprint should place your certain directories and what accessibility they should have.

#### `data.directory`

Private extension-specific files storage. Copied to `.blueprint/extensions/{identifier}/private/`. Blueprint utilizes private directories for internal tracking even without this option enabled.

```yaml [conf.yml]
data:
  # private directory that you can throw extra
  # files in, accessible at {root/data}
  directory: 'path/to/directory'
```

#### `data.public`

Publicly accessible files - gets symlinked to `public/extensions/{identifier}` so anyone can access them via web requests. Don't store anything that shouldn't be accessible to everyone.

```yaml [conf.yml]
data:
  # directory publicly exposed to the internet.
  # accessible internally at {root/public},
  # externally at {webroot/public}
  public: 'path/to/directory'
```

#### `data.console`

Artisan commands and scheduling functionality. Blueprint copies these to `app/Console/Commands/BlueprintFramework/Extensions/{identifier}/` and processes any scheduling configuration through the Laravel console api.

::card
This binding is related to [Console.yml](/docs/configs/consoleyml). Learn more about what it does and how to use it in it's article.
::

```yaml [conf.yml]
data:
  # directory for console commands (Console.yml)
  console: 'path/to/directory'
```

### Requests section

Defines how Blueprint should handle your extension's web request routing and application logic.

#### `requests.views`

Directory containing additional blade view files. Symlinked to `resources/views/blueprint/extensions/{identifier}` for Laravel's view system and accessible as `blueprint.extensions.{identifier}.*`.

```yaml [conf.yml]
requests:
  # directory where you can put uncategorized
  # blade views
  views: 'path/to/directory'
```

#### `requests.app`

Application logic and controllers directory. Symlinked to `app/BlueprintFramework/Extensions/{identifier}`.

```yaml [conf.yml]
requests:
  # add additional php files in here!
  app: 'path/to/directory'
```

#### `requests.routers`

Main Laravel router file. If you only specify this option, Blueprint defaults it to the web router. For more control, use the specific router suboptions.

```yaml [conf.yml]
requests:
  # add a custom (web) routes file
  routers: 'path/to/webroutes.php'
  # -- or --
  routers:
    application: 'path/to/approutes.php'
    client: 'path/to/clientroutes.php'
    web: 'path/to/webroutes.php'
```

##### `requests.routers.application`

Application API routes. Gets copied to `routes/blueprint/application/{identifier}.php` and prefixed by `/api/application/extensions/{identifier}/`. Calling these routes requires a valid application API key.

```yaml [conf.yml]
requests:
  routers:
    # add custom application routes
    # (aka extend the admin/application api)
    application: 'path/to/approutes.php'
```

##### `requests.routers.client`

Client API routes for extending the client api. Copied to `routes/blueprint/client/{identifier}.php` and prefixed by `/api/client/extensions/{identifier}/`. Calling these routes requires a valid client API key.

```yaml [conf.yml]
requests:
  routers:
    # add custom client routes
    # (aka extend the client api)
    client: 'path/to/clientroutes.php'
```

#### `requests.routers.web`

Standard web routes (default when using main `routers` option). Copied to `routes/blueprint/web/{identifier}.php` and prefixed by `/extensions/{identifier}/`.

```yaml [conf.yml]
requests:
  routers:
    # add custom web routes
    # (aka add custom routes prefixed with
    # /extensions/myextension)
    web: 'path/to/webroutes.php'
```

### Database section

Define your extension's database-related bindings.

#### `database.migrations`

Database migration files directory. Blueprint copies these directly to Laravel's `database/migrations/` folder during installation and runs a database migration afterwards.

```yaml [conf.yml]
database:
  # directory with database migrations
  migrations: 'path/to/directory'
```

## Reference template

Want to see the full template instead? Well, here it is. This is a list of all `conf.yml` values currently supported by Blueprint. Some things may shift around over time, but we'll try our best to keep everything backwards-compatible.

```yaml
info:
  name: ''
  identifier: ''
  description: ''
  flags: ''
  version: ''
  target: ''
  author: ''
  icon: ''
  website: ''

admin:
  view: ''
  controller: ''
  css: ''
  wrapper: ''

dashboard:
  css: ''
  wrapper: ''
  components: ''

data:
  directory: ''
  public: ''
  console: ''

requests:
  views: ''
  app: ''
  routers:
    application: ''
    client: ''
    web: ''

database:
  migrations: ''
```
