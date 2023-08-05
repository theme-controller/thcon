---
title: Neovim
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: editor

config_section: neovim
url: https://neovim.io/
setup: |
  ### Install
  Install [thcon.vim](https://github.com/theme-controller/thcon.vim/) with your
  preferred plugin manager. For example, under [packer.nvim](https://github.com/wbthomason/packer.nvim):

  ```diff-lua
   -- ~/.config/nvim/config.lua

   local use = require('packager').use
   require('packer').startup(function()
     use 'tpope/vim-surround'
  +  use { 'theme-controller/thcon.vim', tag = 'v0.5.0' }
   end)
  ```

  ### Load
  Replace your existing colorscheme setting with functions that load thcon-managed
  settings and listen for changes:

  ```diff-lua
   -- ~/.config/nvim/lua/your-appearance.lua
  
  -vim.cmd([[colorscheme evening]])
  +-- Load dynamic config file
  +vim.cmd([[call thcon#load(v:true)]])
  +-- Listen for theme switches and source the appropriate file
  +vim.cmd([[call thcon#listen()]])
  ```

  ### Create configs
  Write a pair of small config files, one for dark-mode settings and one for
  light-mode:

  ```sh
  echo "vim.cmd([[colorscheme evening]])" > ~/.config/nvim/lua/dark.thcon.lua
  echo "vim.cmd([[colorscheme morning]])" > ~/.config/nvim/lua/light.thcon.lua
  ```

  ### Advanced Usage
  Since those files are `source`d by each Neovim instance, they can contain
  any arbitrary lua you'd like. Consider changing statusline themes, adjusting
  indent levels, or whatever other settings you'd like.
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
