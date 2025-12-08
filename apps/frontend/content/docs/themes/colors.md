---
title: Dashboard colors
description: Default Tailwind colors for the user-side dashboard
category: themes
unlisted: true
---

::card
This document has been unlisted following [this blog post](/blog/update-on-themes). Theme color customization only applies to [beta-2025-11](/releases/beta-2025-11) and is no longer supported.
::

::card
A large amount of colors provided on this page originate from the [default Tailwind v3 color palette](https://v3.tailwindcss.com/docs/customizing-colors). We allow modifying these colors through CSS variables.
::

## Introduction

Dashboard color variables allow you to modify Pterodactyl's user-side dashboard palette through a simple set of CSS variables. This document goes over defaults, implementation and relevant guides.

## Default colors

The following colors are the defaults for each color name, which can be used with the `text-<color>` and `bg-<color>` Tailwind classes, and be customized with the `--blueprint-<color>` variables. ([more info about usage](#usage))

:prose-colors{:content='[
{"name": "white", "rgb": "255 255 255"},
{"name": "black", "rgb": "19 26 32"},
{"name": "primary", "rgb": "59 130 246"},
{"name": "neutral", "rgb": "96 109 123"},
{"name": "slate", "rgb": "100 116 139"},
{"name": "gray", "rgb": "96 109 123"},
{"name": "zinc", "rgb": "113 113 122"},
{"name": "stone", "rgb": "120 113 108"},
{"name": "red", "rgb": "239 68 68"},
{"name": "orange", "rgb": "249 115 22"},
{"name": "amber", "rgb": "245 158 11"},
{"name": "yellow", "rgb": "250 204 21"},
{"name": "lime", "rgb": "163 230 53"},
{"name": "green", "rgb": "34 197 94"},
{"name": "emerald", "rgb": "16 185 129"},
{"name": "teal", "rgb": "20 184 166"},
{"name": "cyan", "rgb": "34 211 238"},
{"name": "sky", "rgb": "56 189 248"},
{"name": "blue", "rgb": "59 130 246"},
{"name": "indigo", "rgb": "129 140 248"},
{"name": "violet", "rgb": "139 92 246"},
{"name": "purple", "rgb": "147 51 234"},
{"name": "fuchsia", "rgb": "217 70 239"},
{"name": "pink", "rgb": "236 72 153"},
{"name": "rose", "rgb": "244 63 94"}
]'}

### Brightness

Colors have a set of brightnesses to choose from. These hues can be used with the `text-<color>-<brightness>` and `bg-<color>-<brightness>` Tailwind classes, and be customized with the `--blueprint-<color>-<brightness>` variables. ([more info about usage](#usage))

:prose-color-shades{:content='[
{
"color": "slate",
"hues": [
{"variant": "50", "rgb": "248 250 252"},
{"variant": "100", "rgb": "241 245 249"},
{"variant": "200", "rgb": "226 232 240"},
{"variant": "300", "rgb": "203 213 225"},
{"variant": "400", "rgb": "148 163 184"},
{"variant": "500", "rgb": "100 116 139"},
{"variant": "600", "rgb": "71 85 105"},
{"variant": "700", "rgb": "51 65 85"},
{"variant": "800", "rgb": "30 41 59"},
{"variant": "900", "rgb": "15 23 42"},
{"variant": "950", "rgb": "2 6 23"}
]
},
{
"color": "gray",
"hues": [
{"variant": "50", "rgb": "245 247 250"},
{"variant": "100", "rgb": "229 232 235"},
{"variant": "200", "rgb": "202 209 216"},
{"variant": "300", "rgb": "154 165 177"},
{"variant": "400", "rgb": "123 135 147"},
{"variant": "500", "rgb": "96 109 123"},
{"variant": "600", "rgb": "81 95 108"},
{"variant": "700", "rgb": "63 77 90"},
{"variant": "800", "rgb": "51 64 77"},
{"variant": "900", "rgb": "31 41 51"},
{"variant": "950", "rgb": "3 7 18"}
]
},
{
"color": "zinc",
"hues": [
{"variant": "50", "rgb": "250 250 250"},
{"variant": "100", "rgb": "244 244 245"},
{"variant": "200", "rgb": "228 228 231"},
{"variant": "300", "rgb": "212 212 216"},
{"variant": "400", "rgb": "161 161 170"},
{"variant": "500", "rgb": "113 113 122"},
{"variant": "600", "rgb": "82 82 91"},
{"variant": "700", "rgb": "63 63 70"},
{"variant": "800", "rgb": "39 39 42"},
{"variant": "900", "rgb": "24 24 27"},
{"variant": "950", "rgb": "9 9 11"}
]
},
{
"color": "stone",
"hues": [
{"variant": "50", "rgb": "250 250 249"},
{"variant": "100", "rgb": "245 245 244"},
{"variant": "200", "rgb": "231 229 228"},
{"variant": "300", "rgb": "214 211 209"},
{"variant": "400", "rgb": "168 162 158"},
{"variant": "500", "rgb": "120 113 108"},
{"variant": "600", "rgb": "87 83 78"},
{"variant": "700", "rgb": "68 64 60"},
{"variant": "800", "rgb": "41 37 36"},
{"variant": "900", "rgb": "28 25 23"},
{"variant": "950", "rgb": "12 10 9"}
]
},
{
"color": "red",
"hues": [
{"variant": "50", "rgb": "254 242 242"},
{"variant": "100", "rgb": "254 226 226"},
{"variant": "200", "rgb": "254 202 202"},
{"variant": "300", "rgb": "252 165 165"},
{"variant": "400", "rgb": "248 113 113"},
{"variant": "500", "rgb": "239 68 68"},
{"variant": "600", "rgb": "220 38 38"},
{"variant": "700", "rgb": "185 28 28"},
{"variant": "800", "rgb": "153 27 27"},
{"variant": "900", "rgb": "127 29 29"},
{"variant": "950", "rgb": "69 10 10"}
]
},
{
"color": "orange",
"hues": [
{"variant": "50", "rgb": "255 247 237"},
{"variant": "100", "rgb": "255 237 213"},
{"variant": "200", "rgb": "254 215 170"},
{"variant": "300", "rgb": "253 186 116"},
{"variant": "400", "rgb": "251 146 60"},
{"variant": "500", "rgb": "249 115 22"},
{"variant": "600", "rgb": "234 88 12"},
{"variant": "700", "rgb": "194 65 12"},
{"variant": "800", "rgb": "154 52 18"},
{"variant": "900", "rgb": "124 45 18"},
{"variant": "950", "rgb": "67 20 7"}
]
},
{
"color": "amber",
"hues": [
{"variant": "50", "rgb": "255 251 235"},
{"variant": "100", "rgb": "254 243 199"},
{"variant": "200", "rgb": "253 230 138"},
{"variant": "300", "rgb": "252 211 77"},
{"variant": "400", "rgb": "251 191 36"},
{"variant": "500", "rgb": "245 158 11"},
{"variant": "600", "rgb": "217 119 6"},
{"variant": "700", "rgb": "180 83 9"},
{"variant": "800", "rgb": "146 64 14"},
{"variant": "900", "rgb": "120 53 15"},
{"variant": "950", "rgb": "69 26 3"}
]
},
{
"color": "yellow",
"hues": [
{"variant": "50", "rgb": "254 252 232"},
{"variant": "100", "rgb": "254 249 195"},
{"variant": "200", "rgb": "254 240 138"},
{"variant": "300", "rgb": "253 224 71"},
{"variant": "400", "rgb": "250 204 21"},
{"variant": "500", "rgb": "234 179 8"},
{"variant": "600", "rgb": "202 138 4"},
{"variant": "700", "rgb": "161 98 7"},
{"variant": "800", "rgb": "133 77 14"},
{"variant": "900", "rgb": "113 63 18"},
{"variant": "950", "rgb": "66 32 6"}
]
},
{
"color": "lime",
"hues": [
{"variant": "50", "rgb": "247 254 231"},
{"variant": "100", "rgb": "236 252 203"},
{"variant": "200", "rgb": "217 249 157"},
{"variant": "300", "rgb": "190 242 100"},
{"variant": "400", "rgb": "163 230 53"},
{"variant": "500", "rgb": "132 204 22"},
{"variant": "600", "rgb": "101 163 13"},
{"variant": "700", "rgb": "77 124 15"},
{"variant": "800", "rgb": "63 98 18"},
{"variant": "900", "rgb": "54 83 20"},
{"variant": "950", "rgb": "26 46 5"}
]
},
{
"color": "green",
"hues": [
{"variant": "50", "rgb": "240 253 244"},
{"variant": "100", "rgb": "220 252 231"},
{"variant": "200", "rgb": "187 247 208"},
{"variant": "300", "rgb": "134 239 172"},
{"variant": "400", "rgb": "74 222 128"},
{"variant": "500", "rgb": "34 197 94"},
{"variant": "600", "rgb": "22 163 74"},
{"variant": "700", "rgb": "21 128 61"},
{"variant": "800", "rgb": "22 101 52"},
{"variant": "900", "rgb": "20 83 45"},
{"variant": "950", "rgb": "5 46 22"}
]
},
{
"color": "emerald",
"hues": [
{"variant": "50", "rgb": "236 253 245"},
{"variant": "100", "rgb": "209 250 229"},
{"variant": "200", "rgb": "167 243 208"},
{"variant": "300", "rgb": "110 231 183"},
{"variant": "400", "rgb": "52 211 153"},
{"variant": "500", "rgb": "16 185 129"},
{"variant": "600", "rgb": "5 150 105"},
{"variant": "700", "rgb": "4 120 87"},
{"variant": "800", "rgb": "6 95 70"},
{"variant": "900", "rgb": "6 78 59"},
{"variant": "950", "rgb": "2 44 34"}
]
},
{
"color": "teal",
"hues": [
{"variant": "50", "rgb": "240 253 250"},
{"variant": "100", "rgb": "204 251 241"},
{"variant": "200", "rgb": "153 246 228"},
{"variant": "300", "rgb": "94 234 212"},
{"variant": "400", "rgb": "45 212 191"},
{"variant": "500", "rgb": "20 184 166"},
{"variant": "600", "rgb": "13 148 136"},
{"variant": "700", "rgb": "15 118 110"},
{"variant": "800", "rgb": "17 94 89"},
{"variant": "900", "rgb": "19 78 74"},
{"variant": "950", "rgb": "4 47 46"}
]
},
{
"color": "cyan",
"hues": [
{"variant": "50", "rgb": "236 254 255"},
{"variant": "100", "rgb": "207 250 254"},
{"variant": "200", "rgb": "165 243 252"},
{"variant": "300", "rgb": "103 232 249"},
{"variant": "400", "rgb": "34 211 238"},
{"variant": "500", "rgb": "6 182 212"},
{"variant": "600", "rgb": "8 145 178"},
{"variant": "700", "rgb": "14 116 144"},
{"variant": "800", "rgb": "21 94 117"},
{"variant": "900", "rgb": "22 78 99"},
{"variant": "950", "rgb": "8 51 68"}
]
},
{
"color": "sky",
"hues": [
{"variant": "50", "rgb": "240 249 255"},
{"variant": "100", "rgb": "224 242 254"},
{"variant": "200", "rgb": "186 230 253"},
{"variant": "300", "rgb": "125 211 252"},
{"variant": "400", "rgb": "56 189 248"},
{"variant": "500", "rgb": "14 165 233"},
{"variant": "600", "rgb": "2 132 199"},
{"variant": "700", "rgb": "3 105 161"},
{"variant": "800", "rgb": "7 89 133"},
{"variant": "900", "rgb": "12 74 110"},
{"variant": "950", "rgb": "8 47 73"}
]
},
{
"color": "blue",
"hues": [
{"variant": "50", "rgb": "239 246 255"},
{"variant": "100", "rgb": "219 234 254"},
{"variant": "200", "rgb": "191 219 254"},
{"variant": "300", "rgb": "147 197 253"},
{"variant": "400", "rgb": "96 165 250"},
{"variant": "500", "rgb": "59 130 246"},
{"variant": "600", "rgb": "37 99 235"},
{"variant": "700", "rgb": "29 78 216"},
{"variant": "800", "rgb": "30 64 175"},
{"variant": "900", "rgb": "30 58 138"},
{"variant": "950", "rgb": "23 37 84"}
]
},
{
"color": "indigo",
"hues": [
{"variant": "50", "rgb": "238 242 255"},
{"variant": "100", "rgb": "224 231 255"},
{"variant": "200", "rgb": "199 210 254"},
{"variant": "300", "rgb": "165 180 252"},
{"variant": "400", "rgb": "129 140 248"},
{"variant": "500", "rgb": "99 102 241"},
{"variant": "600", "rgb": "79 70 229"},
{"variant": "700", "rgb": "67 56 202"},
{"variant": "800", "rgb": "55 48 163"},
{"variant": "900", "rgb": "49 46 129"},
{"variant": "950", "rgb": "30 27 75"}
]
},
{
"color": "violet",
"hues": [
{"variant": "50", "rgb": "245 243 255"},
{"variant": "100", "rgb": "237 233 254"},
{"variant": "200", "rgb": "221 214 254"},
{"variant": "300", "rgb": "196 181 253"},
{"variant": "400", "rgb": "167 139 250"},
{"variant": "500", "rgb": "139 92 246"},
{"variant": "600", "rgb": "124 58 237"},
{"variant": "700", "rgb": "109 40 217"},
{"variant": "800", "rgb": "91 33 182"},
{"variant": "900", "rgb": "76 29 149"},
{"variant": "950", "rgb": "46 16 101"}
]
},
{
"color": "purple",
"hues": [
{"variant": "50", "rgb": "245 243 255"},
{"variant": "100", "rgb": "243 232 255"},
{"variant": "200", "rgb": "233 213 255"},
{"variant": "300", "rgb": "216 180 254"},
{"variant": "400", "rgb": "192 132 252"},
{"variant": "500", "rgb": "168 85 247"},
{"variant": "600", "rgb": "147 51 234"},
{"variant": "700", "rgb": "126 34 206"},
{"variant": "800", "rgb": "107 33 168"},
{"variant": "900", "rgb": "88 28 135"},
{"variant": "950", "rgb": "59 7 100"}
]
},
{
"color": "fuchsia",
"hues": [
{"variant": "50", "rgb": "253 244 255"},
{"variant": "100", "rgb": "250 232 255"},
{"variant": "200", "rgb": "245 208 254"},
{"variant": "300", "rgb": "240 171 252"},
{"variant": "400", "rgb": "232 121 249"},
{"variant": "500", "rgb": "217 70 239"},
{"variant": "600", "rgb": "192 38 211"},
{"variant": "700", "rgb": "162 28 175"},
{"variant": "800", "rgb": "134 25 143"},
{"variant": "900", "rgb": "112 26 117"},
{"variant": "950", "rgb": "74 4 78"}
]
},
{
"color": "pink",
"hues": [
{"variant": "50", "rgb": "253 242 248"},
{"variant": "100", "rgb": "252 231 243"},
{"variant": "200", "rgb": "251 207 232"},
{"variant": "300", "rgb": "249 168 212"},
{"variant": "400", "rgb": "244 114 182"},
{"variant": "500", "rgb": "236 72 153"},
{"variant": "600", "rgb": "219 39 119"},
{"variant": "700", "rgb": "190 24 93"},
{"variant": "800", "rgb": "157 23 77"},
{"variant": "900", "rgb": "131 24 67"},
{"variant": "950", "rgb": "80 7 36"}
]
},
{
"color": "rose",
"hues": [
{"variant": "50", "rgb": "255 241 242"},
{"variant": "100", "rgb": "255 228 230"},
{"variant": "200", "rgb": "254 205 211"},
{"variant": "300", "rgb": "253 164 175"},
{"variant": "400", "rgb": "251 113 133"},
{"variant": "500", "rgb": "244 63 94"},
{"variant": "600", "rgb": "225 29 72"},
{"variant": "700", "rgb": "190 18 60"},
{"variant": "800", "rgb": "159 18 57"},
{"variant": "900", "rgb": "136 19 55"},
{"variant": "950", "rgb": "76 5 25"}
]
}
]'}

### Aliased colors

`primary` is, by default, aliased to `blue`.\
`neutral` is, by default, aliased to `gray`.

## Usage

### React components

React components can make use of color classes for both text colors and background colors. Use the `text-<color>` and `bg-<color>` classes as shown below:

```tsx
<p className={'bg-black text-white'}>
  The quick brown fox jumped over the lazy dog.
</p>
```

You may also want to control the brightness of a color:

```tsx
<p className={'bg-gray-900 text-white'}>
  The quick brown fox jumped over the lazy dog.
</p>
```

Or, the opacity (on a scale from 0 to 100) with the `/<opacity>` modifier:

```tsx
<div className={'bg-gray-950'}>
  <div className={'w-30 bg-blue-950'}>
    <span className={'text-blue'}>foo</span>
  </div>
  <span className={'text-blue/50'}>bar</span>
  {/*    changes the opacity -^ */}
</div>
```

### Themes

Themes can adjust these colors through changing `--blueprint-<color>` and `--blueprint-<color>-<brightness>` variables. These rules need to be plain RGB values. CSS color functions are unsupported.

For example, add the following CSS properties to `:root` in your `dashboard.css` file:

```css [dashboard.css]
:root {
  /* set the gray color to tailwind v3's defaults, instead of pterodactyl's */
  --blueprint-gray: var(--blueprint-gray-500);
  --blueprint-gray-50: 248 250 252;
  --blueprint-gray-100: 241 245 249;
  --blueprint-gray-200: 229 231 235;
  --blueprint-gray-300: 203 213 225;
  --blueprint-gray-400: 156 163 175;
  --blueprint-gray-500: 107 114 128;
  --blueprint-gray-600: 75 85 99;
  --blueprint-gray-700: 55 65 81;
  --blueprint-gray-800: 31 41 55;
  --blueprint-gray-900: 17 24 39;
  --blueprint-gray-950: 3 7 18;
}
```

Pterodactyl generally uses the "gray" and "blue" colors for it's UI. Certain parts from the panel (and some extensions) make use of "neutral" and "primary" aliases. These aliases can also be set to their own colors, though it might cause inconsistencies in different parts of the panel if done so.

#### Changing colors for specific elements

Blueprint's color variables can also be adjusted for specific elements:

```css [dashboard.css]
:root {
  /* your global color palette */
}

#NavigationBar {
  /* colors to only apply to an element with the ID "NavigationBar" */
  --blueprint-white: 200 200 200;
}
```
