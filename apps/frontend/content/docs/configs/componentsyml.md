---
title: Components.yml
description: Extend React pages and/or add new ones
category: configs
---

::card
The `Components.yml` configuration lives in the root of an extension's `dashboard.components` directory. You have to create a directory, bind it in [conf.yml](/docs/configs/confyml#dashboardcomponents) and create a `Components.yml` file. All components should be placed in this (or a sub-)directory too.
::

## Introduction

Blueprint's `Components.yml` configuration file allows an extension to add [React components](https://legacy.reactjs.org/tutorial/tutorial.html#what-is-react) to predefined "areas" of the Pterodactyl panel's user frontend. Multiple extensions can extend the same component without conflicting with eachother.

### Why React?

Simply because Pterodactyl's frontend uses React. Blueprint's components implementation extends Pterodactyl's frontend and allows extensions to add components/pages to it.

## Component paths

When referencing components in your Components.yml, Blueprint expects the component's relative (to the `dashboard:components` directory) file path without the file extension. It sounds complicated, but in practice it's quite easy, here are a few examples:

| Path                                                        | Issues (if any)                                                                          |
| ----------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| <code class="text-red-400">../Component.tsx</code>          | Path escapes the `dashboard:components` directory and ends with a file extension         |
| <code class="text-red-400">@/Element</code>                 | Only components provided by the extension's `dashboard:components` directory can be used |
| <code class="text-red-400">ExampleComponent.tsx</code>      | Path ends with with a file extension                                                     |
| <code class="text-green-400">ExampleComponent</code>        | No issues                                                                                |
| <code class="text-green-400">subdirectory/HelloWorld</code> | No issues                                                                                |

## Routes

To define a route, add an entry to the `Navigation.Routes` list in the `Components.yml` configuration.

<!-- prettier-ignore -->
```yaml [Components.yml]
Navigation:
  Routes:
    - { Name: 'Example route', Path: '/exampleroute', Type: 'account', Component: 'MyExampleRoute', AdminOnly: 'false' }
```

Routes have a few configuration options, each with their own purpose and restrictions.

| Option                 | Description                                                                                                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- |
| `Name` (optional)      | The route name visible in the Pterodactyl navigation bar. When left blank, the page will be set as 'unlisted' |
| `Path`                 | URL path for the custom route, relative to `/server/[id]/` or `/account/` depending on the defined `Type`     |
| `Type`                 | Can be either `server` or `account` and determines whether the route is an 'server' route or 'account' route  |
| `Component`            | Path to the route's component                                                                                 |
| `AdminOnly` (optional) | Can be either `true` or `false` and determines whether or not the page is exclusively available to admins     |

### Egg-specific routes

System administrators can set which "eggs" have which routes [as explained in the Manage extensions guide](/guides/admin/extensions#configuring-extensions), with no extension-side implementation needed. Extensions can't predefine on which eggs to display their routes.

### Conflicts

While components themselves are very unlikely to conflict with each other, **routes totally can**. Make sure to not use generic routes names and paths to prevent breakage.

## Configuration

### Example configuration

Here is an example (valid) Components.yml configuration.

<!-- prettier-ignore -->
```yaml [Components.yml]
Navigation:
  NavigationBar:
    BeforeNavigation: 'HelloWorldComponent'
  Routes:
    - { Name: 'My Extension', Path: '/myextension', Type: 'server', Component: 'sections/MyExtensionSection', AdminOnly: 'false' }
    - { Name: 'Security keys', Path: '/securitykeys', Type: 'account', Component: 'sections/SecurityKeysSection', AdminOnly: 'false' }

Account:
  Overfiew:
    BeforeContent: 'UserGreeterComponent'
    AfterContent: 'HelloWorldComponent' #you can use the same component in multiple places!
```

### Reference

As a point of reference, below is a configuration file with all supported "areas", categories and configuration options for Components.yml.

```yaml [Components.yml]
Navigation:
  NavigationBar:
    BeforeNavigation: ''
    AdditionalItems: ''
    AfterNavigation: ''
  SubNavigation:
    BeforeSubNavigation: ''
    AdditionalServerItems: ''
    AdditionalAccountItems: ''
    AfterSubNavigation: ''
  Routes:
    - { Name: '', Path: '', Type: '', Component: '', AdminOnly: '' }

Dashboard:
  Global:
    BeforeSection: ''
    AfterSection: ''
  Serverlist:
    BeforeContent: ''
    AfterContent: ''
    ServerRow:
      BeforeEntryName: ''
      AfterEntryName: ''
      BeforeEntryDescription: ''
      AfterEntryDescription: ''
      ResourceLimits: ''

Authentication:
  Container:
    BeforeContent: ''
    AfterContent: ''

Account:
  Overview:
    BeforeContent: ''
    AfterContent: ''
  API:
    BeforeContent: ''
    AfterContent: ''
  SSH:
    BeforeContent: ''
    AfterContent: ''

Server:
  Terminal:
    BeforeContent: ''
    AdditionalPowerButtons: ''
    BeforeInformation: ''
    AfterInformation: ''
    CommandRow: ''
    AfterContent: ''
  Files:
    Browse:
      BeforeContent: ''
      FileButtons: ''
      DropdownItems: ''
      AfterContent: ''
    Edit:
      BeforeEdit: ''
      AfterEdit: ''
  Databases:
    BeforeContent: ''
    AfterContent: ''
  Schedules:
    List:
      BeforeContent: ''
      AfterContent: ''
    Edit:
      BeforeEdit: ''
      AfterEdit: ''
  Users:
    BeforeContent: ''
    AfterContent: ''
  Backups:
    BeforeContent: ''
    DropdownItems: ''
    AfterContent: ''
  Network:
    BeforeContent: ''
    AfterContent: ''
  Startup:
    BeforeContent: ''
    AfterContent: ''
  Settings:
    BeforeContent: ''
    AfterContent: ''
```
