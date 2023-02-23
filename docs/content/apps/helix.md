---
title: Helix
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: editor

config_section: helix
url: https://helix-editor.com
setup: |
  None needed: Helix themes can be controlled with no setup!
options:
  - key: dark
    default: solarized_dark
    description: Theme name to use in dark mode
  - key: light
    default: solarized_light
    description: Theme name to use in light mode
example: |
  [helix]
  dark = "catppuccin_mocha"
  light = "catppuccin_latte"
---
