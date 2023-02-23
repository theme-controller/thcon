---
title: KDE Plasma
layout: layouts/app.html
platforms: [ linux, freebsd, openbsd ]
category: desktop

config_section: plasma
url: https://kde.org/plasma-desktop/
setup: |
  None needed: KDE Plasma themes can be controlled with no setup!

  For custom themes, run `lookandfeeltool --list` to show all available theme
  packages. Choose the theme packages you want for light and dark mode, and use
  those in package names in `thcon.toml`.
options:
  - key: dark
    default: org.kde.breezedark.desktop
    description: Theme package name to use in dark mode
  - key: light
    default: org.kde.breeze.desktop
    description: Theme package name to use in light mode
example: |
  [plasma]
  dark = com.example.dark.desktop
  light = com.example.light.desktop
---
