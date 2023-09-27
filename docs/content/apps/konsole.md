---
title: Konsole
layout: layouts/app.html
platforms: [ linux, freebsd, openbsd ]
category: terminal

config_section: konsole
needs_config: true
url: https://konsole.kde.org/
setup: |
  `thcon` controls Konsole themes by switching between two profiles you set up
  ahead of time. From the Konsole Settings page, create a pair of profiles and
  configure them to your liking. Use the names of those profiles in
  `thcon.toml`.
options:
  - key: dark
    default: (none)
    description: Profile name to use in dark mode
  - key: light
    default: (none)
    description: Profile name to use in light mode
example: |
  [konsole]
  dark = "My Dark Profile"
  light = "A Light Profile"
---
