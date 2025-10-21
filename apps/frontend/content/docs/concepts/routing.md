---
title: Laravel routes
description: Writing Laravel routes in an extension
category: concepts
---

## Introduction

Blueprint allows extensions to create custom routing configurations for Laravel. Extensions can add custom web routes, but also extend the existing client/application API, without possible conflicts.

### What is the application API?

Pterodactyl's application API requires a key that can be generated from the admin panel. It's generally use for Wings auto-deployments and controlling the administrative-side of Pterodactyl. The application API uses the `/api/application` URL prefix.

### What is the client API?

Pterodactyl's client API is accessible to anyone who is signed in to the panel, or through keys generated in a user's account settings. It allows to have more granular control over servers accessible to the user, which for administrators includes others' servers as well. The client API uses the `/api/client` URL prefix.
