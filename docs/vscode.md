# Visual Studio Code

## Usage: Windows & macOS
Since [version 1.42](https://code.visualstudio.com/updates/v1_42#_auto-switch-theme-based-on-os-color-scheme), Visual Studio Code can listen to Windows and macOS color scheme changes and switch to a matching theme.  It's recommended for use on those platforms.

## Usage: Linux & BSD
Since Visual Studio Code doesn't support OS synchronization on Linux and non-Apple BSDs, `thcon` is the recommended method.  Visual Studio Code monitors its `settings.json` file for changes while it's running.  Because that `settings.json` file can include comments, the simplest way to preserve existing whitespace and comments is by looking for a magic comment annotating the 

In your `settings.json`, mark the `workspace.colorTheme` line so `thcon` can find it:

```jsonc
{
  // ... other settings

  "workbench.colorTheme": "" // thcon:replace-line
}
```

In your `thcon.toml`, define light and dark themes:

```toml
[vscode]
dark = "Default Dark+"   # the default dark theme
light = "Default Light+" # the default light theme

# optionally, tell thcon where your settings.json is stored
config = "/path/to/settings.json"
```

## `thcon.toml` Schema
Section: `vscode`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `dark` | string | The theme to use in dark mode | (none) |
| `light` | string | The theme to use in light mode | (none) |
| `config` | string | Absolute path to your `settings.json` file | `~/.config/Code/User/settings.json` |
