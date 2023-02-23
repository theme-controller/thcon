---
title: Neovim
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: editor

config_section: neovim
url: https://neovim.io/
setup: |
  TODO
options:
  - key: dark
    default: ~/.config/nvim/lua/dark.thcon.lua
    description: Path to the file to `source` for dark mode
  - key: light
    default: ~/.config/nvim/lua/light.thcon.lua
    description: Path to the file to `source` for light mode
example: |
  [neovim]
  dark = "~/Downloads/a-file-from-the-internet.lua"
  light = "~/Documents/light-mode-settings.lua"
---
