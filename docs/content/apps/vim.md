---
title: Vim
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: editor

config_section: vim
needs_config: true
url: https://www.vim.org/
setup: |
  ### Install
  Install [thcon.vim](https://github.com/theme-controller/thcon.vim/) with your
  preferred plugin manager. For example, under [vim-plug](https://github.com/junegunn/vim-plug):

  ```diff-vim
   " ~/.vimrc
   call plug#begin()
  +  Plug 'theme-controller/thcon.vim', { 'tag': 'v0.5.0' }
   call plug#end()
  ```

  ### Load
  At the bottom of your `.vimrc`, replace your `colorscheme` statement with
  commands that load your thcon-managed settings and listen for changes:

  ```diff-vim
   " ~/.vimrc
   " ...
   set number
   set laststatus=2

  -colorscheme evening
  +" Load dynamic config file
  +call thcon#load()
  +" Listen for theme switches and source the appropriate file
  +call thcon#listen()
  ```

  ### Create configs
  Write a pair of small config files, one for dark-mode settings and one for
  light-mode (by default `~/dark.thcon.vimrc` and `~/light.thcon.vimrc`):

  ```sh
  echo "colorscheme evening" > ~/dark.thcon.vimrc
  echo "colorscheme morning" > ~/light.thcon.vimrc
  ```

  ### Advanced Usage
  Since those files are `source`d by each vim instance, they can contain
  any arbitrary Vimscript you'd like. Consider changing statusline themes,
  adjusting indent levels, or whatever other settings you'd like.
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
