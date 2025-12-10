---
title: Reporting bugs and issues
description: Receive support when Blueprint (or an extension) breaks
author: Emma
category: admin
thumbnail: glitch.jpeg
---

## Introduction

Blueprint and extension developers get quite a few "bug reports" that are usually caused by misconfigurations or AI-generated instructions.
This guide aims to help you categorize your support requests and bug reports properly, and link you to the right place for the issue.

### Support channels

We offer community support for both Blueprint and extensions on our platform. If you are experiencing issues with a specific extension, consider contacting the developer directly.

| Purpose                                                          | Location                                                                                                                                                                                                                                                    |
| ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| General community support for Blueprint and extensions           | `#support` in our [Discord community](https://discord.com/servers/blueprint-1063548024825057451) ([Answer Overflow](https://www.answeroverflow.com/c/1063548024825057451)) and [GitHub Discussions](https://github.com/orgs/BlueprintFramework/discussions) |
| Confirmed and reproducible bug reports                           | `blueprintframework/framework`'s [GitHub Issues](https://github.com/BlueprintFramework/framework/issues/new?template=bug-report.yml)                                                                                                                        |
| Confirmed and reproducible bug reports for the Blueprint website | `blueprintframework/web`'s [GitHub Issues](https://github.com/BlueprintFramework/web/issues/new)                                                                                                                                                            |

## Reporting a bug

To prevent spam, please make sure your bug checks off all of the following requirements before submitting a bug report:

- You are able to reproduce the bug and provide instructions on how to do so.
- Your bug report is tested against the latest version of Blueprint.
- You do not have any non-Blueprint modifications installed.
- You exclusively use [Blueprint-verified extensions](/browse).
- Your bug has not already been [reported](https://github.com/BlueprintFramework/framework/issues).

If you meet these requirements, please create an issue [here](https://github.com/BlueprintFramework/framework/issues/new?template=bug-report.yml). Otherwise, make a [support request]().

## Requesting support

Community support is provided for issues you may experience with Blueprint or extensions on our platform.
We provide community support in the `#support` Discord channel ([Discord community](https://discord.com/servers/blueprint-1063548024825057451), [Answer Overflow](https://www.answeroverflow.com/c/1063548024825057451)) and [GitHub Discussions](https://github.com/orgs/BlueprintFramework/discussions).

When creating a support request, please provide sufficient information (like logs and screenshots, for example) for our community to help you properly.
That said, make sure to censor any information that you don't want to be available publicly.

### Collecting panel logs

You can use the following command to share your recent panel logs through a pteropaste link:

```bash
# Path to your Pterodactyl webserver directory
PTERODACTYL_DIRECTORY="/var/www/pterodactyl"

# Takes the 99 most recent panel log lines and sends
# them to pteropaste.com, then returns a share link
tail -n 99 $PTERODACTYL_DIRECTORY/storage/logs/laravel-$(date +%F).log | nc pteropaste.com 99
```

This series of commands will return a link that you can share in Blueprint support channels.
If you prefer not to use pteropaste, remove the `| nc..` part of the command to print out the recent logs in your terminal instead.

### Don't share shell access

Whatever you do, do not share Pterodactyl panel credentials (accounts, api keys) or SSH access.
Not everyone on the internet can be trusted. If anyone asks for this, [please report it to the working group](/legal/conduct).

### Requesting support in the Pterodactyl community

General Pterodactyl-related questions that are not related to Blueprint can be asked in Pterodactyl's support channels.
If you make use of Blueprint and have a more technical question, please ask in our support channels first.

### Support for Blueprint-forks

We do not provide support for **users of Blueprint-forks**. Please search for support methods offered by each fork, then ask your question there.

## Contacting extension developers

Extension developers commonly (not always) provide support for their extensions. If you are experiencing issues (or want to report a bug) with a specific extension, look for their contact details and send them a message.

> Unless mentioned otherwise, please don't send extension developers support requests on support channels like Discord DMs or other instant messaging apps. A lot of extension developers provide support over email or ticketing systems, and don't appreciate getting messaged on other platforms.
