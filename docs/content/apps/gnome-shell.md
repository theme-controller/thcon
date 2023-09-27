---
title: GNOME Shell
layout: layouts/app.html
platforms: [ linux, freebsd, openbsd ]
category: desktop

config_section: gnome-shell
url: https://wiki.gnome.org/Projects/GnomeShell
setup: |
  Ensure the [User Themes extension](https://extensions.gnome.org/extension/19/user-themes/)
  is installed and enabled. Use the name of the theme displayed in the User Themes extension
  config (either via GNOME Extensions or GNOME Tweaks).
options:
  - key: dark
    default: (none)
    description: Theme name to use in dark mode
  - key: light
    default: (none)
    description: Theme name to use in light mode
example: |
  [gnome-shell]
  dark = "Arc-Dark-solid"
  light = "Arc"
---
