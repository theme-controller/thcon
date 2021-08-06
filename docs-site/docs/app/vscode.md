---
title: Visual Studio Code
---

# Visual Studio Code
[Visual Studio Code](https://code.visualstudio.com/) can [automatically switch color schemes](https://code.visualstudio.com/updates/v1_42#_auto-switch-theme-based-on-os-color-scheme) based on your OS theme, and requires a small amount of configuration.  `thcon` isn't required for version 1.59+ on all platforms, or for 1.42+ on Windows or macOS.  If you're stuck on an older version, `thcon` can still help!

## Built-in Support
:::tip REQUIRED VERSIONS
Automatic switching requires at least these versions of Visual Studio Code per-platform:
* Windows: 1.42.0 or above
* macOS: 1.42.0 or above
* Linux/BSD: 1.59.0 or above
:::

Simply set `window.autoDetectColorScheme` to `true` in your `settings.json`, then set `workbench.preferredDarkColorTheme` and `workbench.preferredLightColorTheme` to the names of your preferred themes.  When you switch into and out of your operating system's dark mode, VSCode will automatically switch themes.


```json
{
  // ... other settings

  "window.autoDetectColorScheme": true,
  "workbench.preferredDarkColorTheme": "Solarized Dark",
  "workbench.preferredLightColorTheme": "Solarized Light"
}
```

### Linux/BSD OS Config
VSCode synchronizes "dark mode" to the current GTK theme.  [GTK](./gtk.md) theme switching is enabled by default in `thcon` (including default dark and light themes), but can be configured further if desired.
:::


## Using `thcon` for Older Versions
If you're stuck on an older version of VSCode --- or want more granular control over VSCode themes than synchronizing to the OS would provide --- `thcon` can still help!


Visual Studio Code monitors its `settings.json` file for changes while it's running.  Because that `settings.json` file can include comments, the simplest way to preserve existing whitespace and comments is by looking for a magic comment annotating the `workbench.colorTheme` setting.

In your `settings.json`, mark the `workspace.colorTheme` line so `thcon` can find it, and be sure to disable `window.autoDetectColorScheme`:

```json
{
  // ... other settings

  "window.autoDetectColorScheme": false,
  "workbench.colorTheme": "" // thcon:replace-line
}
```

In your `thcon.toml`, define light and dark themes:

```toml
[vscode]
dark = "Solarized Dark"
light = "Solarized Light"

# optionally, tell thcon where your settings.json is stored
config = "/path/to/settings.json"
```

### Config Schema
Section: `vscode`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `dark` | string | The theme to use in dark mode | Default Dark+ |
| `light` | string | The theme to use in light mode | Default Light+ |
| `config` | string | Absolute path to your `settings.json` file | `~/.config/Code/User/settings.json` |
