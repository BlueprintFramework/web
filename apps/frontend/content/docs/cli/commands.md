---
title: Commands
description: Interact with the Blueprint CLI
category: cli
---

## Extensions

### Install Extension

Install or update one or multiple extensions. Blueprint will automatically determine whether or not to update an extension.

```bash
# Usage
blueprint -install|-add|-i [..name]

# Example
blueprint -install modpackinstaller minecraftplayermanager
```

### Remove Extension

Remove one or more extensions from your panel.

```bash
# Usage
blueprint -remove|-r [..name]

# Example
blueprint -remove mcplugins cats
```

### Query Extension

Query extension metadata.

```bash
# Usage
blueprint -query|-q [name]

# Example
blueprint -query nebula
```

## Developer

### Initialize

Initialize Blueprint extension development files. This command generates and prefills development files in `.blueprint/dev` based on a template.

```bash
# Usage
blueprint -init|-I

# Example
blueprint -init
```

### Build

Install or update your extension development files, as if you were installing it as a packaged extension.

```bash
# Usage
blueprint -build|-b

# Example
blueprint -build
```

### Watch

Watch for changes and automatically rebuild your extension development files.

```bash
# Usage
blueprint -watch

# Example
blueprint -watch
```

### Regenerate dist

Rebuild extension development dist directory, primarily used for type safety in IDEs.

```bash
# Usage
blueprint -dist

# Example
blueprint -dist
```

### Export

Package your extension and optionally generate a temporary download link through the `expose` argument.

```bash
# Usage
blueprint -export|-e ('expose')

# Example
blueprint -export
blueprint -export expose
```

### Wipe

Permanently deletes all development files stored in `.blueprint/dev`.

```bash
# Usage
blueprint -wipe|-w

# Example
blueprint -wipe
```

## Misc

### Version

Returns the installed Blueprint version.

```bash
# Usage
blueprint -version|-v

# Example
blueprint -version
```

### Help

Displays the CLI's help menu.

```bash
# Usage
blueprint -help|-h

# Example
blueprint -help
```

### Info

Print information about your Blueprint installation, structured in a neofetch-like format.

```bash
# Usage
blueprint -info|-f

# Example
blueprint -info
```

### Debug

Print given amount of debug lines.

```bash
# Usage
blueprint -debug [lines]

# Example
blueprint -debug 100
```

## Advanced

### Upgrade

Update or reset to another release, optionally a GitHub repository.

```bash
# Usage
blueprint -upgrade (remote) (github-repo)

# Example
blueprint -upgrade
blueprint -upgrade remote blueprintframework/framework
```

### Rerun Install

Rerun the Blueprint installation script.

```bash
# Usage
blueprint -rerun-install

# Example
blueprint -rerun-install
```
