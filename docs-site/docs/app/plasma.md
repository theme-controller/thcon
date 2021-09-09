# KDE Plasma

::: tip NO CONFIGURATION REQUIRED
`thcon` can switch between the default KDE Plasma light and dark themes immediately after installation with no
configuration.
:::

## Usage: Linux & BSD
KDE Plasma already ships with a commandline tool to switch global UI themes: `lookandfeeltool`.  `thcon` simply shells out to that command, so configuring `thcon` requires a brief interaction with it if you don't like the default themes.

Run `lookandfeeltool --list` to show all available theme packages. Choose the theme packages you want for light and dark mode, then list those in your `thcon.toml`, e.g.:

```toml
[plasma]
dark = "org.kde.breezedark.desktop"   # the default dark theme
light = "org.kde.breeze.desktop"      # the default light theme
```

## Usage: Windows & macOS
KDE Plasma is not supported on these platforms.

## Config Schema
Section: `plasma`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `dark` | string | The theme package name to use in dark mode | `org.kde.breezedark.desktop` |
| `light` | string | The theme package name to use in light mode | `org.kde.breeze.desktop` |
