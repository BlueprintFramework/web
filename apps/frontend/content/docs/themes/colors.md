---
title: Dashboard colors
description: Default Tailwind colors for the user-side dashboard
category: themes
---

::card
A large amount of colors provided on this page originate from the [default Tailwind v3 color palette](https://v3.tailwindcss.com/docs/customizing-colors). We allow modifying these colors through CSS variables.
::

## Introduction

Dashboard color variables allow you to modify Pterodactyl's user-side dashboard palette through a simple set of CSS variables. This document goes over defaults, implementation and relevant guides.

## Default colors

The following colors are the defaults for each color name, these can be used with the `text-<color-name>` and `bg-<color-name>` Tailwind classes, and be customized with the `--blueprint-<color-name>` variables.

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

### Color palette

Each color (excluding white and black), have a set of hues to choose from.

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

### Slate

:prose-colors{:content='[{"name": "slate", "rgb": "100 116 139"}, {"name": "slate-50", "rgb": "248 250 252"}, {"name": "slate-100", "rgb": "241 245 249"}, {"name": "slate-200", "rgb": "226 232 240"}, {"name": "slate-300", "rgb": "203 213 225"}, {"name": "slate-400", "rgb": "148 163 184"}, {"name": "slate-500", "rgb": "100 116 139"}, {"name": "slate-600", "rgb": "71 85 105"}, {"name": "slate-700", "rgb": "51 65 85"}, {"name": "slate-800", "rgb": "30 41 59"}, {"name": "slate-900", "rgb": "15 23 42"}, {"name": "slate-950", "rgb": "2 6 23"}]'}

### Gray

:prose-colors{:content='[{"name": "gray", "rgb": "96 109 123"}, {"name": "gray-50", "rgb": "245 247 250"}, {"name": "gray-100", "rgb": "229 232 235"}, {"name": "gray-200", "rgb": "202 209 216"}, {"name": "gray-300", "rgb": "154 165 177"}, {"name": "gray-400", "rgb": "123 135 147"}, {"name": "gray-500", "rgb": "96 109 123"}, {"name": "gray-600", "rgb": "81 95 108"}, {"name": "gray-700", "rgb": "63 77 90"}, {"name": "gray-800", "rgb": "51 64 77"}, {"name": "gray-900", "rgb": "31 41 51"}, {"name": "gray-950", "rgb": "3 7 18"}]'}

### Zinc

:prose-colors{:content='[{"name": "zinc", "rgb": "113 113 122"}, {"name": "zinc-50", "rgb": "250 250 250"}, {"name": "zinc-100", "rgb": "244 244 245"}, {"name": "zinc-200", "rgb": "228 228 231"}, {"name": "zinc-300", "rgb": "212 212 216"}, {"name": "zinc-400", "rgb": "161 161 170"}, {"name": "zinc-500", "rgb": "113 113 122"}, {"name": "zinc-600", "rgb": "82 82 91"}, {"name": "zinc-700", "rgb": "63 63 70"}, {"name": "zinc-800", "rgb": "39 39 42"}, {"name": "zinc-900", "rgb": "24 24 27"}, {"name": "zinc-950", "rgb": "9 9 11"}]'}

### Stone

:prose-colors{:content='[{"name": "stone", "rgb": "120 113 108"}, {"name": "stone-50", "rgb": "250 250 249"}, {"name": "stone-100", "rgb": "245 245 244"}, {"name": "stone-200", "rgb": "231 229 228"}, {"name": "stone-300", "rgb": "214 211 209"}, {"name": "stone-400", "rgb": "168 162 158"}, {"name": "stone-500", "rgb": "120 113 108"}, {"name": "stone-600", "rgb": "87 83 78"}, {"name": "stone-700", "rgb": "68 64 60"}, {"name": "stone-800", "rgb": "41 37 36"}, {"name": "stone-900", "rgb": "28 25 23"}, {"name": "stone-950", "rgb": "12 10 9"}]'}

### Red

:prose-colors{:content='[{"name": "red", "rgb": "239 68 68"}, {"name": "red-50", "rgb": "254 242 242"}, {"name": "red-100", "rgb": "254 226 226"}, {"name": "red-200", "rgb": "254 202 202"}, {"name": "red-300", "rgb": "252 165 165"}, {"name": "red-400", "rgb": "248 113 113"}, {"name": "red-500", "rgb": "239 68 68"}, {"name": "red-600", "rgb": "220 38 38"}, {"name": "red-700", "rgb": "185 28 28"}, {"name": "red-800", "rgb": "153 27 27"}, {"name": "red-900", "rgb": "127 29 29"}, {"name": "red-950", "rgb": "69 10 10"}]'}

### Orange

:prose-colors{:content='[{"name": "orange", "rgb": "249 115 22"}, {"name": "orange-50", "rgb": "255 247 237"}, {"name": "orange-100", "rgb": "255 237 213"}, {"name": "orange-200", "rgb": "254 215 170"}, {"name": "orange-300", "rgb": "253 186 116"}, {"name": "orange-400", "rgb": "251 146 60"}, {"name": "orange-500", "rgb": "249 115 22"}, {"name": "orange-600", "rgb": "234 88 12"}, {"name": "orange-700", "rgb": "194 65 12"}, {"name": "orange-800", "rgb": "154 52 18"}, {"name": "orange-900", "rgb": "124 45 18"}, {"name": "orange-950", "rgb": "67 20 7"}]'}

### Amber

:prose-colors{:content='[{"name": "amber", "rgb": "245 158 11"}, {"name": "amber-50", "rgb": "255 251 235"}, {"name": "amber-100", "rgb": "254 243 199"}, {"name": "amber-200", "rgb": "253 230 138"}, {"name": "amber-300", "rgb": "252 211 77"}, {"name": "amber-400", "rgb": "251 191 36"}, {"name": "amber-500", "rgb": "245 158 11"}, {"name": "amber-600", "rgb": "217 119 6"}, {"name": "amber-700", "rgb": "180 83 9"}, {"name": "amber-800", "rgb": "146 64 14"}, {"name": "amber-900", "rgb": "120 53 15"}, {"name": "amber-950", "rgb": "69 26 3"}]'}

### Yellow

:prose-colors{:content='[{"name": "yellow", "rgb": "250 204 21"}, {"name": "yellow-50", "rgb": "254 252 232"}, {"name": "yellow-100", "rgb": "254 249 195"}, {"name": "yellow-200", "rgb": "254 240 138"}, {"name": "yellow-300", "rgb": "253 224 71"}, {"name": "yellow-400", "rgb": "250 204 21"}, {"name": "yellow-500", "rgb": "234 179 8"}, {"name": "yellow-600", "rgb": "202 138 4"}, {"name": "yellow-700", "rgb": "161 98 7"}, {"name": "yellow-800", "rgb": "133 77 14"}, {"name": "yellow-900", "rgb": "113 63 18"}, {"name": "yellow-950", "rgb": "66 32 6"}]'}

### Lime

:prose-colors{:content='[{"name": "lime", "rgb": "163 230 53"}, {"name": "lime-50", "rgb": "247 254 231"}, {"name": "lime-100", "rgb": "236 252 203"}, {"name": "lime-200", "rgb": "217 249 157"}, {"name": "lime-300", "rgb": "190 242 100"}, {"name": "lime-400", "rgb": "163 230 53"}, {"name": "lime-500", "rgb": "132 204 22"}, {"name": "lime-600", "rgb": "101 163 13"}, {"name": "lime-700", "rgb": "77 124 15"}, {"name": "lime-800", "rgb": "63 98 18"}, {"name": "lime-900", "rgb": "54 83 20"}, {"name": "lime-950", "rgb": "26 46 5"}]'}

### Green

:prose-colors{:content='[{"name": "green", "rgb": "34 197 94"}, {"name": "green-50", "rgb": "240 253 244"}, {"name": "green-100", "rgb": "220 252 231"}, {"name": "green-200", "rgb": "187 247 208"}, {"name": "green-300", "rgb": "134 239 172"}, {"name": "green-400", "rgb": "74 222 128"}, {"name": "green-500", "rgb": "34 197 94"}, {"name": "green-600", "rgb": "22 163 74"}, {"name": "green-700", "rgb": "21 128 61"}, {"name": "green-800", "rgb": "22 101 52"}, {"name": "green-900", "rgb": "20 83 45"}, {"name": "green-950", "rgb": "5 46 22"}]'}

### Emerald

:prose-colors{:content='[{"name": "emerald", "rgb": "16 185 129"}, {"name": "emerald-50", "rgb": "236 253 245"}, {"name": "emerald-100", "rgb": "209 250 229"}, {"name": "emerald-200", "rgb": "167 243 208"}, {"name": "emerald-300", "rgb": "110 231 183"}, {"name": "emerald-400", "rgb": "52 211 153"}, {"name": "emerald-500", "rgb": "16 185 129"}, {"name": "emerald-600", "rgb": "5 150 105"}, {"name": "emerald-700", "rgb": "4 120 87"}, {"name": "emerald-800", "rgb": "6 95 70"}, {"name": "emerald-900", "rgb": "6 78 59"}, {"name": "emerald-950", "rgb": "2 44 34"}]'}

### Teal

:prose-colors{:content='[{"name": "teal", "rgb": "20 184 166"}, {"name": "teal-50", "rgb": "240 253 250"}, {"name": "teal-100", "rgb": "204 251 241"}, {"name": "teal-200", "rgb": "153 246 228"}, {"name": "teal-300", "rgb": "94 234 212"}, {"name": "teal-400", "rgb": "45 212 191"}, {"name": "teal-500", "rgb": "20 184 166"}, {"name": "teal-600", "rgb": "13 148 136"}, {"name": "teal-700", "rgb": "15 118 110"}, {"name": "teal-800", "rgb": "17 94 89"}, {"name": "teal-900", "rgb": "19 78 74"}, {"name": "teal-950", "rgb": "4 47 46"}]'}

### Cyan

:prose-colors{:content='[{"name": "cyan", "rgb": "34 211 238"}, {"name": "cyan-50", "rgb": "236 254 255"}, {"name": "cyan-100", "rgb": "207 250 254"}, {"name": "cyan-200", "rgb": "165 243 252"}, {"name": "cyan-300", "rgb": "103 232 249"}, {"name": "cyan-400", "rgb": "34 211 238"}, {"name": "cyan-500", "rgb": "6 182 212"}, {"name": "cyan-600", "rgb": "8 145 178"}, {"name": "cyan-700", "rgb": "14 116 144"}, {"name": "cyan-800", "rgb": "21 94 117"}, {"name": "cyan-900", "rgb": "22 78 99"}, {"name": "cyan-950", "rgb": "8 51 68"}]'}

### Sky

:prose-colors{:content='[{"name": "sky", "rgb": "56 189 248"}, {"name": "sky-50", "rgb": "240 249 255"}, {"name": "sky-100", "rgb": "224 242 254"}, {"name": "sky-200", "rgb": "186 230 253"}, {"name": "sky-300", "rgb": "125 211 252"}, {"name": "sky-400", "rgb": "56 189 248"}, {"name": "sky-500", "rgb": "14 165 233"}, {"name": "sky-600", "rgb": "2 132 199"}, {"name": "sky-700", "rgb": "3 105 161"}, {"name": "sky-800", "rgb": "7 89 133"}, {"name": "sky-900", "rgb": "12 74 110"}, {"name": "sky-950", "rgb": "8 47 73"}]'}

### Blue

:prose-colors{:content='[{"name": "blue", "rgb": "59 130 246"}, {"name": "blue-50", "rgb": "239 246 255"}, {"name": "blue-100", "rgb": "219 234 254"}, {"name": "blue-200", "rgb": "191 219 254"}, {"name": "blue-300", "rgb": "147 197 253"}, {"name": "blue-400", "rgb": "96 165 250"}, {"name": "blue-500", "rgb": "59 130 246"}, {"name": "blue-600", "rgb": "37 99 235"}, {"name": "blue-700", "rgb": "29 78 216"}, {"name": "blue-800", "rgb": "30 64 175"}, {"name": "blue-900", "rgb": "30 58 138"}, {"name": "blue-950", "rgb": "23 37 84"}]'}

### Indigo

:prose-colors{:content='[{"name": "indigo", "rgb": "129 140 248"}, {"name": "indigo-50", "rgb": "238 242 255"}, {"name": "indigo-100", "rgb": "224 231 255"}, {"name": "indigo-200", "rgb": "199 210 254"}, {"name": "indigo-300", "rgb": "165 180 252"}, {"name": "indigo-400", "rgb": "129 140 248"}, {"name": "indigo-500", "rgb": "99 102 241"}, {"name": "indigo-600", "rgb": "79 70 229"}, {"name": "indigo-700", "rgb": "67 56 202"}, {"name": "indigo-800", "rgb": "55 48 163"}, {"name": "indigo-900", "rgb": "49 46 129"}, {"name": "indigo-950", "rgb": "30 27 75"}]'}

### Violet

:prose-colors{:content='[{"name": "violet", "rgb": "139 92 246"}, {"name": "violet-50", "rgb": "245 243 255"}, {"name": "violet-100", "rgb": "237 233 254"}, {"name": "violet-200", "rgb": "221 214 254"}, {"name": "violet-300", "rgb": "196 181 253"}, {"name": "violet-400", "rgb": "167 139 250"}, {"name": "violet-500", "rgb": "139 92 246"}, {"name": "violet-600", "rgb": "124 58 237"}, {"name": "violet-700", "rgb": "109 40 217"}, {"name": "violet-800", "rgb": "91 33 182"}, {"name": "violet-900", "rgb": "76 29 149"}, {"name": "violet-950", "rgb": "46 16 101"}]'}

### Purple

:prose-colors{:content='[{"name": "purple", "rgb": "147 51 234"}, {"name": "purple-50", "rgb": "245 243 255"}, {"name": "purple-100", "rgb": "243 232 255"}, {"name": "purple-200", "rgb": "233 213 255"}, {"name": "purple-300", "rgb": "216 180 254"}, {"name": "purple-400", "rgb": "192 132 252"}, {"name": "purple-500", "rgb": "168 85 247"}, {"name": "purple-600", "rgb": "147 51 234"}, {"name": "purple-700", "rgb": "126 34 206"}, {"name": "purple-800", "rgb": "107 33 168"}, {"name": "purple-900", "rgb": "88 28 135"}, {"name": "purple-950", "rgb": "59 7 100"}]'}

### Fuchsia

:prose-colors{:content='[{"name": "fuchsia", "rgb": "217 70 239"}, {"name": "fuchsia-50", "rgb": "253 244 255"}, {"name": "fuchsia-100", "rgb": "250 232 255"}, {"name": "fuchsia-200", "rgb": "245 208 254"}, {"name": "fuchsia-300", "rgb": "240 171 252"}, {"name": "fuchsia-400", "rgb": "232 121 249"}, {"name": "fuchsia-500", "rgb": "217 70 239"}, {"name": "fuchsia-600", "rgb": "192 38 211"}, {"name": "fuchsia-700", "rgb": "162 28 175"}, {"name": "fuchsia-800", "rgb": "134 25 143"}, {"name": "fuchsia-900", "rgb": "112 26 117"}, {"name": "fuchsia-950", "rgb": "74 4 78"}]'}

### Pink

:prose-colors{:content='[{"name": "pink", "rgb": "236 72 153"}, {"name": "pink-50", "rgb": "253 242 248"}, {"name": "pink-100", "rgb": "252 231 243"}, {"name": "pink-200", "rgb": "251 207 232"}, {"name": "pink-300", "rgb": "249 168 212"}, {"name": "pink-400", "rgb": "244 114 182"}, {"name": "pink-500", "rgb": "236 72 153"}, {"name": "pink-600", "rgb": "219 39 119"}, {"name": "pink-700", "rgb": "190 24 93"}, {"name": "pink-800", "rgb": "157 23 77"}, {"name": "pink-900", "rgb": "131 24 67"}, {"name": "pink-950", "rgb": "80 7 36"}]'}

### Rose

:prose-colors{:content='[{"name": "rose", "rgb": "244 63 94"}, {"name": "rose-50", "rgb": "255 241 242"}, {"name": "rose-100", "rgb": "255 228 230"}, {"name": "rose-200", "rgb": "254 205 211"}, {"name": "rose-300", "rgb": "253 164 175"}, {"name": "rose-400", "rgb": "251 113 133"}, {"name": "rose-500", "rgb": "244 63 94"}, {"name": "rose-600", "rgb": "225 29 72"}, {"name": "rose-700", "rgb": "190 18 60"}, {"name": "rose-800", "rgb": "159 18 57"}, {"name": "rose-900", "rgb": "136 19 55"}, {"name": "rose-950", "rgb": "76 5 25"}]'}
