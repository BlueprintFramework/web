---
title: Commands
description: Interact with the Blueprint CLI
category: cli
---

## Extensions

### Install Extension

Install or update a Blueprint extension

```bash
# Usage
blueprint -install|-add|-i [..name]

# Example
blueprint -install modpackinstaller minecraftplayermanager
```

### Remove Extension

Remove a Blueprint extension

```bash
# Usage
blueprint -remove|-r [..name]

# Example
blueprint -remove mcplugins cats
```

### Query Extension

Get information about a specific Blueprint extension

```bash
# Usage
blueprint -query|-q [name]

# Example
blueprint -query nebula
```

## Developer

### Initialize

Initialize Blueprint extension development files

```bash
# Usage
blueprint -init|-I

# Example
blueprint -init
```

### Build

Install or update your extension development files

```bash
# Usage
blueprint -build|-b

# Example
blueprint -build
```

### Watch

Watch for changes and automatically rebuild your extension development files

```bash
# Usage
blueprint -watch

# Example
blueprint -watch
```

### Regenerate dist

Rebuild extension development dist directory

```bash
# Usage
blueprint -dist

# Example
blueprint -dist
```

### Export

Package your extension and optionally generate a temporary download link

```bash
# Usage
blueprint -export|-e ('expose')

# Example
blueprint -export
blueprint -export expose
```

### Wipe

Remove your extension development files

```bash
# Usage
blueprint -wipe|-w

# Example
blueprint -wipe
```

## Misc

### Version

Returns the Blueprint installation's version

```bash
# Usage
blueprint -version|-v

# Example
blueprint -version
```

### Help

Displays the CLI help menu

```bash
# Usage
blueprint -help|-h

# Example
blueprint -help
```

### Info

Print neofetch-like information about your Blueprint installation

```bash
# Usage
blueprint -info|-f

# Example
blueprint -info
```

### Debug

Print given amount of debug lines

```bash
# Usage
blueprint -debug [lines]

# Example
blueprint -debug 100
```

## Advanced

### Upgrade

Update or reset to another release

```bash
# Usage
blueprint -upgrade (remote) (github-repo)

# Example
blueprint -upgrade
blueprint -upgrade remote blueprintframework/framework
```

### Rerun Install

Rerun the Blueprint installation script

```bash
# Usage
blueprint -rerun-install

# Example
blueprint -rerun-install
```
