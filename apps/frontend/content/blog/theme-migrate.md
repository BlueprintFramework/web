---
title: Migrate from twin.macro's theme()
description: A step in the right direction for Blueprint themes
author: Emma
thumbnail: theme-migrate.jpg
date: 12/7/2025 20:06 (CEDST)
num: 1
---

> Before you ask; Blueprint themes are not going anywhere. This is purely a change to how extensions fetch theme color values through React components.

---

We're making the difficult decision to stop supporting twin.macro's `theme()` utility. Starting beta-2025-12, calling this function will return values that may differ from the Pterodactyl theme used on the panel.

In the [beta-2025-11 release](/releases/beta-2025-11), we brought new theming capabilities to Blueprint. The implementation was quite rocky however, and only worked properly on a small amount of installations. After weeks of trying, we think the only way forward is deprecating and introducing an alternative to twin.macro's theme colors utility.

### Migrating to Blueprint's theme() function

If you are currently making use of twin.macro's `theme()` utility in your extension, you should move over to Blueprint's.

You only need to adjust the import. The utility itself will function (largely) the same.

```diff
- import { theme } from 'twin.macro'
+ import { theme } from '@blueprint'
```

### What if I don't migrate?

**Your extension will not stop working**. Instead, theme values fetched through this function may no longer accurately reflect Pterodactyl's theme. For theme colors, it may return `#000000` instead of the correct HEX value.
