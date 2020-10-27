# Alacritty Config
Since [alacritty](https://github.com/alacritty/alacritty) is configured via [yaml](https://yaml.org/), using anchors and aliases is the simplest way of managing color schemes.

## Usage
In your `alacritty.yml`, define your colors

```yaml
# define your color themes:

solarized: &solarized_dark
  #         ^^^^^^^^^^^^^^ - use this name in thcon.toml
  primary:
    background: '0x002b36'
    foreground: '0x839496'
  # ... the normal contents of a `colors` object

light_solarized: &solarized_light:
  #               ^^^^^^^^^^^^^^^ - use this name in thcon.toml
  primary:
    background: '0xfdf6e3'
    foreground: '0x586e75'

# then choose your color scheme one last time:
colors: *solarized_light # thcon:replace-line

# thcon will manage the line ending in `thcon:replace-line`
# to swap alacritty color schemes
```

In your `thcon.toml`, define light and dark themes based on the `&anchor`s defined above:

```toml
[alacritty]
dark = "solarized_dark"
light = "solarized_light"

# optionally, tell thcon where your alacritty config is stored
config = "/path/to/alacritty.yml"
```

## `thcon.toml` Schema
Section: `alacritty`

| Key | Type | Description | Defaults |
| --- | ---- | ----------- | -------- |
| `dark` | string | The YAML anchor (declared in `alacritty.yml`) used for dark mode | (none) |
| `light` | string | The YAML anchor (declared in `alacritty.yml`) used for light mode | (none) |
| `config` | string | Absolute path to your `alacritty.yml` file | (see below) |

### Default value for `config`
Thcon checks all default locations that `alacritty` [defines for alacritty.yml](https://github.com/alacritty/alacritty#configuration):

* Windows: `%APPDATA%\alacritty\alacritty.yml`
* Other platforms:
  1. `$XDG_CONFIG_HOME/alacritty/alacritty.yml`
  2. `$XDG_CONFIG_HOME/alacritty.yml`
  3. `$HOME/.config/alacritty/alacritty.yml`
  4. `$HOME/.alacritty.yml`