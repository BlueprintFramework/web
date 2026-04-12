# Working on frontend

Thanks for contributing to Blueprint's web platform, this is how you get a development environment up and running.

## Things you need

- A supported version of Node.js
- PNPM for managing dependencies
- computer
- internet

## Setting up the dev environment

Navigate to the `apps/frontend/` directory with `cd apps/frontend` if you haven't already, then proceed with the steps documented below :)

### Choose your API

You can choose to use either the production API or run the backend locally. Either option can be used to

Please make sure you have a working dev environment for the Blueprint web platform backend. The frontend is quite dependent on the API, and if it's down, it will explode.

> [!INFO] You can use the production API!\
> If you are not working with the API, you can proxy your `/api` requests to the production one. Please note, however, that you may not be able to make any calls that require a captcha.
>
> You can set the following value in [`nuxt.config.ts`](./nuxt.config.ts), though please don't stage these changes in your PRs:
>
> ```diff
>
> ```

```

### set up dependencies

1. Copy [`pnpm-workspace.example.yaml`](./pnpm-workspace.example.yaml) to [`pnpm-workspace.yaml`](./pnpm-workspace.yaml)
```
