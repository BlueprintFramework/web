---
version: alpha-F0V
released: 2023-09-19T17:56:38Z
num: 1
---

#### Changes

- Add multiple comments to `blueprint.sh` to make understanding what happens easier for contributors.
- Blueprint now stops installing extensions that have folder paths defined that end with a slash as that would lead to unexpected errors.
- Move all Blueprint libraries into a new "BlueprintFramework" folder in the app directory.
- Improve notification API by relying on the database instead of local files.
- Errors related to dependencies are now grouped nicely into one error instead of one error per dependency.
- Add new telemetry messages.

#### Bugfixes

- Some characters were detected as "yes" when running `-upgrade`.
- ~~When providing wrong answers in -init, your new answers would be behind your rejected answers in your conf.yml file. Instead of the final variable being "goodanswer" it would be "rejectedanswergoodanswer" when having any rejected values.~~
- Admin layout (also referred to as admin wrapper in [conf.yml](/docs/configs/confyml)) only had access to [$blueprint](/docs/lib/methods) if the admin page had [$blueprint](/docs/lib/methods) imported, this is now fixed.
- Some users were required to refresh the Pterodactyl route cache every time Blueprint modified it to apply router changes, this is now done automatically.
- We experienced a vague [BlueprintExtensionLibrary](/docs/lib/methods) error on the authentication page on the client panel, importing the library directly in the view should have fixed this.

#### Breaking changes

- BlueprintExtensionLibrary has changed location, name and namespace. Older extensions that do not update and change to this namespace will stay working for the next few updates but will not receive updates that come to the extension library.
