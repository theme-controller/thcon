---
title: GNOME Terminal
layout: layouts/app.html
platforms: [ linux, freebsd, openbsd ]
category: terminal

config_section: gnome-terminal
needs_config: true
url: https://wiki.gnome.org/Apps/Terminal
setup: |
  `thcon` controls GNOME Terminal themes by switching between two profiles you
  set up ahead of time. From the GNOME Terminal Preferences page, create a pair
  of profiles and configure them to your liking. Use the IDs of those profiles
  (shown in the bottom corner of the preferences window) in your `thcon.toml`
  file.

  <picture>
    <source srcset="/gnome-terminal-preferences.avif" type="image/avif" width="848" height="661">
    <source srcset="/gnome-terminal-preferences.webp" type="image/webp" width="848" height="661">
    <img
      src="/public/gnome-terminal-preferences.png"
      type="image/png"
      width="848"
      height="661"
      alt="Screenshot of the GNOME Terminal Preferences window.">
  </picture>
options:
  - key: dark
    default: (none)
    description: The ID of the profile (case-sensitive) to use in dark mode
  - key: light
    default: (none)
    description: The ID of the profile (case-sensitive) to use in light mode
example: |
  [gnome-terminal]
  dark = "193ae12e-5755-42bf-b10e-a1a428016b4a"
  light = "fe1bf46e-4e61-4389-bd93-f0000a7e8ac6"
---
