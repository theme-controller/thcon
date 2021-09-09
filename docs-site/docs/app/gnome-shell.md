# GNOME Shell

::: warning GNOME EXTENSION REQUIRED
GNOME Shell user themes require the [User Themes extension](https://extensions.gnome.org/extension/19/user-themes/) to be enabled.
:::

## Usage: Linux & BSD
With the [User Themes extension](https://extensions.gnome.org/extension/19/user-themes/) installed and enabled, simply provide the name of the theme as displayed in the User Themes extension config (either via GNOME Extensions or GNOME Tweaks), e.g.:

```toml
[gnome-shell]
light = "Arc"
dark = "Arc-Dark-solid"
```

## Usage: Windows & macOS
Currently unsupported.

## Config Schema
Section: `gnome-shell`

| Key | Type | Description | Default |
| --- | ---- | ----------- | ------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `dark` | string | The name of the theme (case-sensitive) to apply in dark mode | (none) |
| `light` | string | The name of the theme (case-sensitive) to apply in light mode | (none) |
