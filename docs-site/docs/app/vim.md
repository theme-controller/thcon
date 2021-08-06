---
title: Vim & Neovim
---

# Vim & Neovim

::: warning PLUGIN REQUIRED
Most Vim and Neovim distributions include a light and dark theme by default, but all distributions require [a plugin](https://github.com/theme-controller/thcon-vim) to interact with `thcon`.
:::

## Windows
Windows is not yet supported by `thon`, but `vim`/`nvim` under WSL should work just fine.

## macOS & Linux
Install [thcon.vim](https://github.com/theme-controller/thcon.vim) via your `.vimrc` or `init.vim`
using your preferred plugin manager ([dein.vim](https://github.com/Shougo/dein.vim) and [vim-plug](https://github.com/junegunn/vim-plug) are popular options).  Lower in your init script, add these lines to load and detect changes from `thcon`:

```vim
call thcon#load()   " load previously-applied settings as defaults
call thcon#listen() " listen for new settings to be applied with `thcon`
```

In your `thcon.toml`, define light and dark themes. All values within 'dark' and 'light' are
optional (blank values cause no changes):

```toml
[vim]
light.colorscheme = "shine"
dark.colorscheme = "blue"
```

or:

```toml
[vim.light]
colorscheme = "shine"

[vim.dark]
colorscheme = "blue"
```

or:

```toml
[neovim]
dark.colorscheme = "default"
dark.set.background = "dark"
dark.let."g:lightline" = { colorscheme = "ayu_dark" }
light.colorscheme = "shine"
light.set.background = "light"
light.let."g:lightline" = { colorscheme = "ayu_light" }
```

Feel free to use whichever syntax you prefer &mdash; or any other &mdash; as long as it's valid TOML.

## `thcon.toml` Schema
Section: `vim` or `nvim`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `light` | table | Settings to apply in light mode | (none) |
| `light.colorscheme` | string | The colorscheme to apply in light mode | (none) |
| `light.set` | table | Set of key/value pairs to apply with `:set` in light mode | (none) |
| `light.setglobal` | table | Set of key/value pairs to apply with `:setglobal` in light mode | (none) |
| `light.let` | table | Set of key/value pairs to apply with `:let` in light mode | (none) |
| `dark` | table | Settings to apply in dark mode | (none) |
| `dark.colorscheme` | string | The colorscheme to apply in dark mode | (none) |
| `dark.set` | table | Set of key/value pairs to apply with `:set` in dark mode | (none) |
| `dark.setglobal` | table | Set of key/value pairs to apply with `:setglobal` in dark mode | (none) |
| `dark.let` | table | Set of key/value pairs to apply with `:let` in dark mode | (none) |

