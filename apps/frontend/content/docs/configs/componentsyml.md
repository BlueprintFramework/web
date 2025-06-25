---
title: Components.yml
description: Extend React pages and/or add new ones
category: configs
---

```yaml [Components.yml]
Navigation:
  NavigationBar:
    BeforeNavigation:
    AdditionalItems:
    AfterNavigation:
  SubNavigation:
    BeforeSubNavigation:
    AdditionalServerItems:
    AdditionalAccountItems:
    AfterSubNavigation:
  Routes:
  # - { Name: "", Path: "", Type: "account/server", Component: "", AdminOnly: "boolean" }

Dashboard:
  Global:
    BeforeSection:
    AfterSection:
  Serverlist:
    BeforeContent:
    AfterContent:
    ServerRow:
      BeforeEntryName:
      AfterEntryName:
      BeforeEntryDescription:
      AfterEntryDescription:
      ResourceLimits:

Authentication:
  Container:
    BeforeContent:
    AfterContent:

Account:
  Overview:
    BeforeContent:
    AfterContent:
  API:
    BeforeContent:
    AfterContent:
  SSH:
    BeforeContent:
    AfterContent:

Server:
  Terminal:
    BeforeContent:
    AdditionalPowerButtons:
    BeforeInformation:
    AfterInformation:
    CommandRow:
    AfterContent:
  Files:
    Browse:
      BeforeContent:
      FileButtons:
      DropdownItems:
      AfterContent:
    Edit:
      BeforeEdit:
      AfterEdit:
  Databases:
    BeforeContent:
    AfterContent:
  Schedules:
    List:
      BeforeContent:
      AfterContent:
    Edit:
      BeforeEdit:
      AfterEdit:
  Users:
    BeforeContent:
    AfterContent:
  Backups:
    BeforeContent:
    DropdownItems:
    AfterContent:
  Network:
    BeforeContent:
    AfterContent:
  Startup:
    BeforeContent:
    AfterContent:
  Settings:
    BeforeContent:
    AfterContent:
```
