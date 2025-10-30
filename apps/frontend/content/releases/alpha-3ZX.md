---
version: alpha-3ZX
released: 2023-11-26T14:33:20Z
num: 5
---

#### Changes

- New installation timestamp placeholder: `^#timestamp#^`
- New public directory placeholder: `^#publicpath#^`
- Add website button icon for Patreon, Reddit, Trello, GitLab and more.
- Uninstallation script has been added that allows extensions to remove extension files that Blueprint cannot keep track of.
- New asset: `logoCoverTextBlue.jpg`.
- Import [Bootstrap Icons](https://icons.getbootstrap.com) to the admin page by default.
- Developers now see a warning log when using templates on older, outdated versions of Blueprint.
- Update related links in `README.md` and update extensions list in both `README.md` and `extensions.md`.
- Developers can now add export scripts using the new `hasExportScript` flag, allowing for making modifications that do not impact your source code.
- New throwError library for `blueprint.sh`, which will be used in the future for easier exception logging.
- Improve code readability on multiple custom internal libraries that are utilized by `blueprint.sh`.
- No longer use market variables for version names.
- Add `POST` route for extensions which will trigger a `post()` function in the extension's admin controller.

#### Bugfixes

- Incorrect website button icon for Discord extension website links.
- Database values were not being synced when running the install script for the first time after upgrading.
- "Unknown file or directory" errors every time Blueprint logs something when the debug log file is missing.
- Remove stray .gitkeep file in `.blueprint/extensions/`.
