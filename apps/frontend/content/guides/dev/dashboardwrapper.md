---
title: Extending the user dashboard wrapper
description: Extend the Pterodactyl user dashboard using the Laravel blade wrapper
author: Emma
category: dev
thumbnail: dashboard.jpeg
---

::card
The dashboard wrapper should not be your first choice for dashboard extensibility. It's quite limited and can cause weird issues with the React DOM. Use [Components.yml](/docs/configs/componentsyml) instead if available for your extension's use case.
::

## Introduction

Blueprint's dashboard wrapper bind extends the user-side dashboard
