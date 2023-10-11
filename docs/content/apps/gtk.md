---
title: GTK
layout: layouts/app.html
platforms: [ linux, freebsd, openbsd ]
category: desktop

config_section: gtk
needs_config: false
url: https://gtk.org/
setup: |
  `thcon` controls GTK3 and GTK4 themes that would otherwise be managed by
  [dconf](https://developer.gnome.org/dconf/unstable/dconf-overview.html)
  via [GNOME Tweaks](https://wiki.gnome.org/Apps/Tweaks) or the
  [KDE GTK Configurator](https://invent.kde.org/plasma/kde-gtk-config).

  If you have GNOME Tweaks installed, use the name of your dark mode and light
  mode themes as shown in GNOME Tweaks. If you don't, use the filename for that
  theme as it exists on-disk in `/usr/themes/`, `/usr/local/themes/`, or
  `~/.themes/`. Invalid values default to `Adwaita` (light mode).
options:
  - key: dark
    default: Adwaita-dark
    description: The name of the theme (case-sensitive) to apply in dark mode
  - key: light
    default: Adwaita
    description: The name of the theme (case-sensitive) to apply in light mode
example: |
  [gtk]
  dark = "HighContrastInverse"
  light = "HighContrast"
---
