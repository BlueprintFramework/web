---
version: alpha-2N2
released: 2023-10-07T18:32:16Z
summary: That time we gave yall an excuse to not ship extension icons
num: 2
---

#### Changes

- Update assets: `defaultExtensionLogo1`, `defaultExtensionLogo2`, `defaultExtensionLogo3`, `defaultExtensionLogo4`, `defaultExtensionLogo5`, `defaultExtensionLogo6`.
- Add new assets: `installWithBlueprintBanner`, `defaultExtensionLogo7`, `defaultExtensionLogo8`, `defaultExtensionLogo9`.
- Add templates for creating development extensions. Extension developers can now start directly from a template or use a barebones one.
- Add new `-wipe` command for developers. This allows developers to remove their development files.
- Add shorter versions of developer commands that you can use alongside normal developer commands.
- Link Laravel filesystems when installing/upgrading Blueprint.
- Make the extension management page's buttons both nicer and responsive.
- The Blueprint admin page is now responsive, this means better usability of mobile users.
- Improve readability in `blueprint.sh` and add comments detailing what some code does.
- Add some additional instructions for installing extensions in the `README.md` file to prevent confusion.

#### Bugfixes

- Stacking rejected `-init` answers were supposed to be fixed in `alpha-F0V`, but was fixed incorrectly.
- Including file extensions when running `-install` no longer runs into a "not found" error.
- Extensions with no extension icon could not be installed even though icons were already optional.
- Route removal was not working as expected when removing extensions.
