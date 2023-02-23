---
title: Vim
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: editor

config_section: vim
url: https://www.vim.org/
setup: |
  TODO
options:
  - key: dark
    default: ~/dark.thcon.vimrc
    description: Path to the file to `source` for dark mode
  - key: light
    default: ~/light.thcon.vimrc
    description: Path to the file to `source` for light mode
example: |
  [vim]
  dark = "~/.config/vim/my-dark-settings.vimrc"
  light = "~/.config/vim/my-light-settings.vimrc"
---
