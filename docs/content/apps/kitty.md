---
title: Kitty
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: terminal

config_section: kitty
url: https://sw.kovidgoyal.net/kitty/
setup: |
  No additional plugins are required to control kitty themes but there is a
  tiny amount of setup:

  1. Split your dark mode and light mode themes into separate `.conf` files (or use
  [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes))
  2. Use the paths to those files in `thcon.toml` (examples below)
  3. Ensure you don't have any other [color settings](https://sw.kovidgoyal.net/kitty/conf/#color-scheme)
     in your main `kitty.conf`
  4. Add `include ~/.local/share/thcon/kitty.conf` to your main `kitty.conf`
options:
  - key: dark
    default: ~/.config/kitty/dark.thcon.conf
    description: Path to the file defining dark mode settings
  - key: light
    default: ~/.config/kitty/light.thcon.conf
    description: Path to the file defining light mode settings
example: |
  [kitty]
  dark = ~/src/iTerm2-Color-Schemes/kitty/catppuccin-mocha.conf
  light = ~/src/iTerm2-Color-Schemes/kitty/catppuccin-latte.conf
---
