---
title: Alacritty
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: terminal

config_section: alacritty
needs_config: true
url: https://alacritty.org
setup: |
  No additional plugins are required to control alacritty themes but there is a
  tiny amount of setup. Alacritty uses different config languages before/after
  0.13.0:

  <h3 id="yaml">YAML Config (< v0.13.0) <a class="permalink" title="permalink" href="#yaml">&para;</a></h3>
  <details open>
    <summary>Setup instructions</summary>

  1. Split your dark mode and light mode themes into separate `.yml` files, or
     skip this step and use pre-defined themes from
     [alacritty-theme](https://github.com/alacritty/alacritty-theme) or 
     [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes):<br/>
     `~/.config/alacritty/{dark,light}.thcon.yml`
  2. Use the paths to those files in `thcon.toml` (examples below)
  3. Replace the [color settings](https://github.com/alacritty/alacritty/blob/9d9982df0ed3ab40a9b432e8e8b75d9c7f7bd882/alacritty.yml#L295-L339)
     in your main `alacritty.yml` with a single [import](https://github.com/alacritty/alacritty/blob/9d9982df0ed3ab40a9b432e8e8b75d9c7f7bd882/alacritty.yml#L3-L12)
     entry for `~/.local/share/thcon/alacritty.yml`

  In other words:

  ```diff-yaml
   # alacritty.yml

   import:
  +  - ~/.local/share/thcon/alacritty.yml

   colors:
     cursor:
       text: CellBackground
       cursor: CellForeground
  -  primary:
  -    background: '#1d1f21'
  -    foreground: '#c5c8c6'
  -  normal:
  -    black: '#1d1f21'
  -    red: '#cc6666'
  -    # ...
  -   bright:
  -    black: '#666666'
  -    red: '#d54e53'
  -    # ...
  -   dim:
  -    black: '#131415'
  -    red: '#864343'
  -    # ...
  ```

  And if you're not using pre-defined theme files, make sure those exist:

  ```yaml
  # Skip if you're using pre-defined themes
  #
  # ~/.config/alacritty/dark.thcon.yml
  # Colors (Solarized Dark)
  colors:
    # Default colors
    primary:
      background: '0x002b36'
      foreground: '0x839496'
    # ...
  ```

  ```yaml
  # Skip if you're using pre-defined themes
  #
  # ~/.config/alacritty/light.thcon.yml
  # Colors (Solarized Light)
  colors:
    # Default colors
    primary:
      background: '0xfdf6e3'
      foreground: '0x586e75'
    # ...
  ```
  </details>

  

  <h3 id="toml">TOML Config (>= v0.13.0) <a class="permalink" title="permalink" href="#toml">&para;</a></h3>
  <details>
    <summary>Setup instructions</summary>

  1. Split your dark mode and light mode themes into separate `.toml` files, or
     skip this step and use pre-defined themes from
     [alacritty-theme](https://github.com/alacritty/alacritty-theme) or 
     [iTerm2-Color-Schemes](https://github.com/mbadolato/iTerm2-Color-Schemes):<br/>
     `~/.config/alacritty/{dark,light}.thcon.toml`
  2. Use the paths to those files in `thcon.toml` (examples below)
  3. Replace the [color settings](https://github.com/alacritty/alacritty/blob/77aa9f42bac4377efe26512d71098d21b9b547fd/extra/man/alacritty.5.scd#colors)
     in your main `alacritty.toml` with a single [import](https://github.com/alacritty/alacritty/blob/77aa9f42bac4377efe26512d71098d21b9b547fd/extra/man/alacritty.5.scd#general)
     entry for `~/.local/share/thcon/alacritty.yml`

  In other words:

  ```diff-toml
   # alacritty.toml

   [general]
  +import = [ "~/.local/share/thcon/alacritty.yml" ]

   [colors.cursor]
   text = "CellBackground"
   cursor = "CellForeground"

  -[colors.primary]
  -background = "#1d1f21"
  -foreground = "#c5c8c6"
  -
  -[colors.normal]
  -black = "#1d1f21"
  -red = "#cc6666"
  -# ...
  -
  -[colors.bright]
  -black = "#666666"
  -red = "#d54e53"
  -# ...
  -
  -[colors.dim]
  -black = "#131415"
  -red = "#864343"
  -# ...
  ```

  And if you're not using pre-defined theme files, make sure those exist:

  ```toml
  # Skip if you're using pre-defined themes
  #
  # ~/.config/alacritty/dark.thcon.toml
  # Colors (Solarized Dark)
  [colors.primary]
  background = "0x002b36"
  foreground = "0x839496"
  # ...
  ```

  ```toml
  # Skip if you're using pre-defined themes
  #
  # ~/.config/alacritty/light.thcon.toml
  # Colors (Solarized Light)
  [colors.primary]
  background = "0xfdf6e3"
  foreground = "0x586e75"
  # ...
  ```
  </details>

  

options:
  - key: dark
    default: (none)
    description: Path to the file defining dark mode settings
  - key: light
    default: (none)
    description: Path to the file defining light mode settings
example: |
  # For alacritty < 0.13.0, reference YAML files:
  [alacritty]
  dark = "~/config/alacritty/themes/themes/solarized_dark.yaml"
  light = "~/config/alacritty/themes/themes/solarized_light.yaml"

  # For alacritty >= 0.13.0, references TOML files:
  [alacritty]
  dark = "~/Downloads/solarized_dark.toml"
  light = "~/Downloads/solarized_light.toml"
---
