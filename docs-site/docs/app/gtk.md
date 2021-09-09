# GTK

## Usage: Linux & BSD
The active GTK application theme is managed by [dconf](https://developer.gnome.org/dconf/unstable/dconf-overview.html), and is typically accessed as a user via something like [Gnome Tweaks](https://wiki.gnome.org/Apps/Tweaks) or [KDE GTK Configurator](https://invent.kde.org/plasma/kde-gtk-config).  `thcon` can manage GTK theme switching quite simply by reading the desired theme names from `thcon.toml`, e.g.:

```toml
[gtk]
dark = HighContrastInverse
light = HighContrast
```
The value should be the name of the desired theme as reported in Gnome Tweaks, or its filename in `/usr/themes/`, `/usr/local/themes/` or `~/.themes/` if you don't have Gnome Tweaks installed.  Invalid values default to `Adwaita` (light mode).

## Usage: Windows & macOS
Currently unsupported.

## Config Schema
Section: `gtk`

| Key | Type | Description | Default |
| --- | ---- | ----------- | ------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `dark` | string | The name of the theme (case-sensitive) to apply in dark mode | `Adwaita-dark` |
| `light` | string | The name of the theme (case-sensitive) to apply in light mode | `Adwaita` |

