# Working on frontend

Thanks for contributing to Blueprint's web platform, this is how you get a development environment up and running.

> [!TIP]
> Need help? [We'll be happy to help through one of our support channels.](https://blueprint.zip/guides/admin/reporting-issues)

## Things you need

- A supported version of Node.js
- [pnpm](https://pnpm.io) for managing dependencies
- computer
- internet

## Setting up the dev environment

Navigate to the `apps/frontend/` directory with `cd apps/frontend` if you haven't already, then proceed with the steps documented below :)

### Copy example files

Copy the example files to their correct paths. Please inspect these files beforehand, in case any of them require configuration.

```bash
cp .env.example .env
cp pnpm-workspace.example.yaml pnpm-workspace.yaml
```

### Install node dependencies

The frontend is built with Nuxt, among other dependencies. We need those to be installed.

> [!TIP]
> We require dependency releases to be at least 1 day old in when building for production. This is defined in your [`pnpm-workspace.yaml`](./pnpm-workspace.example.yaml). It's recommended to keep this setting as-is.

> [!NOTE]
> Please do not update frontend dependencies unless it's the sole purpose of your PR.

Use the following command to install node dependencies for this project. Make sure you are in the `apps/frontend/` directory.

```bash
pnpm install
```

### Choose your API

You can choose to use either the production API or run the backend locally. Either option can be used, though each have their own up and downsides. If you don't set up either, the website may explode.

#### Option 1: Production API

The easiest option, but also the most limiting. You can choose to use the production API to avoid having to run the backend locally.

- You will not be able to test admin actions if you don't have an admin account in production.
- All actions you perform actually happen on-site, so be careful.
- You may be unable to perform actions that require a captcha.
- API paths return the same data as they do on the production site, which is good if you want to preview changes to the extension browser, for example.

To set this up, change the following in [`nuxt.config.ts`](./nuxt.config.ts). Please do not commit these changes in your PRs.

```diff
nitro: {
  devProxy: {
    '/api': {
-     target: 'http://localhost:8000/api',
+     target: 'https://blueprint.zip/api',
      changeOrigin: true,
    },
  },
},
```

Now you can move onto the next step!

#### Option 2: Locally hosted API

Please set up the [backend environment](../backend/), and make sure to host it on `localhost:8000`. Then, move onto the next step.

### Run the dev server

Last but not least, run the web server. Your changes will now preview on `localhost:3000`.

```bash
pnpm dev
```
