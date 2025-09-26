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

**Type:** string

display name shown to administrators in the panel. gets processed through placeholder replacement system, so you can include special characters but be careful with yaml escaping.

```yaml [conf.yml]
info:
  # the name of your extension, write
  # something creative! you can always change
  # it later
  name: 'My extension'
```

#### `info.identifier` (required)

**Type:** unique `a-z` string

differentiates your extension from others - used for folder trees, routers, view names, internal tracking, etc. blueprint stores this in `.blueprint/extensions/blueprint/private/db/installed_extensions` and uses it throughout the filesystem structure. must be unique per system and only contain lowercase a-z characters.

```yaml [conf.yml]
info:
  # the unique identifier of your extension,
  # you can't really (easily) change this later
  identifier: 'myextension'
```

#### `info.description` (required)

**Type:** string

extension description shown to administrators on your extension's admin page. also supports placeholder replacement.

```yaml [conf.yml]
info:
  # shown to administrators on the extension's
  # admin page
  description: 'This is an awesome extension!'
```

#### `info.flags`

**Type:** comma-separated flags

optional list of extension flags for advanced functionality. common flags include `ignorePlaceholders` to disable placeholder replacement, `show_in_sidebar` to add extension button to admin sidebar, and various developer escalation flags. blueprint processes these during the `assignflags` function call.

```yaml [conf.yml]
info:
  # advanced and experimental flags. don't know
  # what these are? you probably don't need them
  flags: 'flag1, flag2, flag3'
```

#### `info.version` (required)

**Type:** string

your extension's current version. used for update detection and displayed in admin panels.

```yaml [conf.yml]
info:
  # the extension's version
  version: '1.0'
```

#### `info.target` (required)

**Type:** string

blueprint version your extension targets. blueprint compares this against the current version during installation and warns users if versions don't match, but still allows installation. helps users understand compatibility.

```yaml [conf.yml]
info:
  # when this doesn't match a panel's blueprint
  # version, they may see a warning about it
  target: 'beta-2025-09'
```

#### `info.author`

**Type:** string

your name/team name - shows up in extension listings. will be required once blueprint's extension metadata system is fully implemented.

```yaml [conf.yml]
info:
  # put your own name here
  author: 'Nothing To Be Seen Here Industries'
```

#### `info.icon`

**Type:** image file

extension icon for admin panel. supports `.jpg`/`.jpeg`, `.svg`, `.png`, `.gif`, `.webp`. blueprint uses a random placeholder icon if this is blank. gets copied to the assets folder during installation.

```yaml [conf.yml]
info:
  # icons can be jpg/jpeg, svg, png, gif or webp!!
  icon: 'path/to/icon.png'
```

#### `info.website`

**Type:** url

website link displayed on extension's admin page. blueprint automatically adds http/https scheme if missing during installation.

```yaml [conf.yml]
info:
  # some urls have different icons on the extension's
  # admin page :)
  website: 'https://prpl.wtf'
```

### Admin section

controls how your extension integrates with pterodactyl's admin panel. blueprint creates admin routes and controllers based on these settings.

#### `admin.view` (required)

**Type:** `.blade.php` file

path to your admin blade view. blueprint copies this to `resources/views/admin/extensions/{identifier}/` and creates the appropriate route structure. this is the main interface administrators will see.

```yaml [conf.yml]
admin:
  # blade views are basically html with support for
  # php logic!
  view: 'path/to/view.blade.php'
```

#### `admin.controller`

**Type:** `.php` file

path to custom admin view controller. if not defined, blueprint uses its built-in controller which provides basic functionality. gets copied to `app/Http/Controllers/Admin/Extensions/{identifier}/` if specified.

```yaml [conf.yml]
admin:
  # when kept empty, blueprint will use a basic
  # built-in controller
  controller: 'path/to/controller.php'
```

#### `admin.css`

**Type:** `.css` file

custom css for pterodactyl admin panel. blueprint processes this file and imports it into the main admin stylesheet via `@import url(/assets/extensions/{identifier}/admin.style.css);`

```yaml [conf.yml]
admin:
  # plain css added into the admin panel
  css: 'path/to/admin.css'
```

#### `admin.wrapper`

**Type:** `.blade.php` file

extends admin panel layout with additional blade content. gets copied to `resources/views/blueprint/admin/wrappers/{identifier}.blade.php` and included in the admin layout.

```yaml [conf.yml]
admin:
  # some additional html/php added to the end of
  # the admin panel
  wrapper: 'path/to/wrapper.blade.php'
```

### Dashboard section

controls integration with pterodactyl's client-facing dashboard. works similarly to admin section but for end users.

#### `dashboard.css`

**type:** `.css` file
custom css for pterodactyl client panel. gets processed and imported into the client stylesheet system.

```yaml [conf.yml]
dashboard:
  # css added to the pterodactyl dashboard
  css: 'path/to/dashboard.css'
```

#### `dashboard.wrapper`

**type:** `.blade.php` file
extends client dashboard layout with additional blade content. copied to `resources/views/blueprint/dashboard/wrappers/{identifier}.blade.php`.

```yaml [conf.yml]
dashboard:
  # html/php added to the end of the frontend page,
  # outside of the react bundle
  wrapper: 'path/to/wrapper.blade.php'
```

#### `dashboard.components`

**type:** directory path
react components for direct pterodactyl frontend integration. blueprint processes the `Components.yml` configuration file in this directory and integrates your react components into the frontend build system.

```yaml [conf.yml]
dashboard:
  # directory for components (Components.yml)
  components: 'path/to/directory'
```

### Data section

defines where blueprint should place your extension's files and what accessibility they should have.

#### `data.directory`

**type:** directory path
private extension-specific files storage. copied to `.blueprint/extensions/{identifier}/private/`. blueprint utilizes private directories for internal tracking even without this option enabled.

```yaml [conf.yml]
data:
  # private directory that you can throw extra
  # files in, accessible at {root/data}
  directory: 'path/to/directory'
```

#### `data.public`

**type:** directory path
publicly accessible files - gets symlinked to `public/extensions/{identifier}` so anyone can access them via web requests. don't store secrets/keys here since they'll be world-readable.

```yaml [conf.yml]
data:
  # directory publicly exposed to the internet.
  # accessible internally at {root/public},
  # externally at {webroot/public}
  public: 'path/to/directory'
```

#### `data.console`

**type:** directory path
artisan commands and scheduling functionality. blueprint copies these to `app/Console/Commands/BlueprintFramework/Extensions/{identifier}/` and processes any scheduling configuration through the console api.

```yaml [conf.yml]
data:
  # directory for console commands (Console.yml)
  console: 'path/to/directory'
```

### Requests section

defines how blueprint should handle your extension's web request routing and application logic.

#### `requests.views`

**type:** directory path
directory containing your blade view files. gets copied to `.blueprint/extensions/{identifier}/views/` and symlinked to `resources/views/blueprint/extensions/{identifier}` for laravel's view system.

```yaml [conf.yml]
requests:
  # directory where you can put uncategorized
  # blade views
  views: 'path/to/directory'
```

#### `requests.app`

**type:** directory path
application logic and controllers directory. copied to `.blueprint/extensions/{identifier}/app/` and symlinked to `app/BlueprintFramework/Extensions/{identifier}` so laravel can autoload your classes.

```yaml [conf.yml]
requests:
  # add additional php files in here!
  app: 'path/to/directory'
```

#### `requests.routers`

**type:** `<router>.php` file or use suboptions below
main router file. if you only specify this option, blueprint defaults it to the web router. for more control, use the specific router suboptions.

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

**type:** `<application_router>.php` file
application-specific api routes. gets copied to `routes/blueprint/application/{identifier}.php` and included in laravel's api route group.

```yaml [conf.yml]
requests:
  routers:
    # add custom application routes
    # (aka extend the admin/application api)
    application: 'path/to/approutes.php'
```

##### `requests.routers.client`

**type:** `<client_router>.php` file
client-facing routes for your extension's frontend functionality. copied to `routes/blueprint/client/{identifier}.php`.

```yaml [conf.yml]
requests:
  routers:
    # add custom client routes
    # (aka extend the client api)
    client: 'path/to/clientroutes.php'
```

#### `requests.routers.web`

**type:** `<web_router>.php` file
standard web routes (default when using main `routers` option). copied to `routes/blueprint/web/{identifier}.php` and included in laravel's web route group.

```yaml [conf.yml]
requests:
  routers:
    # add custom web routes
    # (aka add custom routes prefixed with
    # /extensions/myextension)
    web: 'path/to/webroutes.php'
```

### Database section

handles database-related functionality for your extension.

#### `database.migrations`

**type:** directory path
database migration files directory. blueprint copies these directly to laravel's `database/migrations/` folder during installation, so they'll run with the next `php artisan migrate` command.

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
