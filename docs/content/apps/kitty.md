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

  1. Split your dark mode and light mode themes into separate `.conf` files, or
     skip this step and use pre-defined themes from
     [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes):
     `~/.config/kitty/{dark,light}.thcon.conf`
  2. Use the paths to those files in `thcon.toml` (examples below)
  3. Replace the [color settings](https://sw.kovidgoyal.net/kitty/conf/#color-scheme)
     in your main `kitty.conf` with
     <code>include&nbsp;~/.local/share/thcon/kitty.conf</code>

  In other words:

  ```diff-text
   # kitty.conf

  -background #002b36
  -foreground #839496
  -# ...
  +include ~/.local/share/thcon/kitty.conf
  ```

  ```text
  # ~/.config/kitty/dark.thcon.conf

  background #002b36
  foreground #839496
  # ...
  ```

  ```text
  # ~/.config/kitty/light.thcon.conf

  background #fdf6e3
  foreground #657b83
  # ...
  ```

options:
  - key: dark
    default: (none)
    description: Path to the file defining dark mode settings
  - key: light
    default: (none)
    description: Path to the file defining light mode settings
example: |
  [kitty]
  dark = ~/src/iTerm2-Color-Schemes/kitty/catppuccin-mocha.conf
  light = ~/src/iTerm2-Color-Schemes/kitty/catppuccin-latte.conf
---
