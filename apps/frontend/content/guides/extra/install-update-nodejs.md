---
title: How to install Node.js on Ubuntu/Debian
description: Install or update/downgrade Node.js to use it for Pterodactyl and Blueprint
author: Emma
category: extra
thumbnail: nodejs.jpeg
---

## Introduction

[Node.js](https://nodejs.org) is a JavaScript (ECMAScript) runtime.
It allows you to run JavaScript code outside of your web browser.
Node.js, and it's alternatives, are widely used across the web development space.

Node.js itself, however, is usually not provided through your system's official software repositories.
Even if it's provided, it's usually still more viable to use tools like [Fast Node Manager](https://github.com/Schniz/fnm) (fnm) as it allows you to quickly switch between Node.js versions.

Node.js provides an [**official installation guide**](https://nodejs.org/en/download) on their website.

## Install Node.js

Blueprint requires Node version `v20.x` or later, though using the latest LTS release (which is `v24.x` at the time of writing this guide) is more viable long-term.

We'll use [Fast Node Manager](https://github.com/Schniz/fnm) (fnm) to quickly install the `v24.x` version of Node.js.

```bash
# Download and install fnm
curl -o- https://fnm.vercel.app/install | bash

# Download and install Node.js v24.x using fnm
fnm install 24

# Returns your node version
node -v
```

### Install the `yarn` package manager for Pterodactyl

Blueprint and Pterodactyl require the `yarn` package manager to be used for managing node modules. You can install `yarn` like so:

```bash
# Install yarn
npm i yarn -g

# Returns your yarn version
yarn -v
```

Now you can use `yarn` to manage Pterodactyl's node modules.

## Update and downgrade Node.js

You can use [Fast Node Manager](https://github.com/Schniz/fnm) (fnm) to change the active version, like so:

```bash
# Use Node.js v24.x
fnm use 24

# Use Node.js v22.x
fnm use 22
```

Then you can check your Node.js version with the following command:

```bash
# Returns your node version
node -v
```

## Remove Node.js

Finally, you can remove Node.js versions from your machine through [Fast Node Manager](https://github.com/Schniz/fnm) (fnm) with the following command:

```bash
# Uninstalls Node.js v24.x
fnm uninstall 24
```
