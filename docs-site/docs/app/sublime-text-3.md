---
title: Sublime Text 3
---

# Sublime Text 3

:::danger DISABLED BY DEFAULT
Sublime Text 4 can automatically sync your UI theme and colorscheme to your OS!  To avoid overwriting your settings, `thcon` disables Sublime Text 3 configuration by default.  Read on for Sublime Text 3 configuration, or see [Sublime Text 4](./sublime-text-4.md) for details on ST4's synchronized switching.
:::

## Aside: Comments and sublime-settings order
Changing through the Sublime Text UI any setting that can appear in `Preferences.sublime-settings` causes that file to be completely rewritten. This causes `// ...` comments to be completely removed, and results in keys that are sorted alphabetically. `thcon` matches this behavior.

## Usage
Sublime Text monitors its `Preferences.sublime-settings` file for changes while it's running, applying changes as they appear. `thcon` will parse that file, replace the `theme` and `color_scheme` values (if values are provided in `thcon.toml`), and write the new file back in-place. Copy the `color_scheme` and `theme` values from your `Preferences.sublime-settings` into `thcon.toml`:

```toml
[sublime-text]
disabled = false
# (optional) tell `thcon` where your preferences are if they're not in the default location
# preferences = /path/to/your/Preferences.sublime-settings

[sublime-text.dark]
color_scheme = "Packages/Color Scheme - Default/Monokai.sublime-color-scheme"
theme = "Default.sublime-theme"

[sublime-text.light]
color_scheme = "Packages/Color Scheme - Default/Celeste.sublime-color-scheme"
theme = "Adaptive.sublime-theme"
```

## Config Schema
Section: `sublime-text`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `true` |
| `light` | table | Settings to apply in light mode | |
| `light.color_scheme` | string | The `color_scheme` to use in light mode | `Packages/Color Scheme - Default/Celeste.sublime-color-scheme` |
| `light.theme` | string | The `theme` to use in light mode | `Adaptive.sublime-theme` |
| `dark` | table | Settings to apply in dark mode | |
| `light.color_scheme` | string | The `color_scheme` to use in dark mode | `Packages/Color Scheme - Default/Monokai.sublime-color-scheme` |
| `light.theme` | string | The `theme` to use in dark mode | `Default.sublime-theme` |
| `preferences` | string | Absolute path to your `Preferences.sublime-settings` file | Default Sublime Text 3 locations: <ul><li>Linux/BSD: `~/.config/sublime-text-3/Packages/User/Preferences.sublime-settings`</li><li>macOS: `~/Library/Application Support/Sublime Text 3/Packages/User/Preferences.sublime-settings`</li></ul> |
