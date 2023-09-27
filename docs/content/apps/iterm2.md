---
title: iTerm2
layout: layouts/app.html
platforms: [ macos ]
category: terminal

config_section: iterm2
needs_config: true
url: https://iterm2.com
setup: |
  `thcon` controls iTerm2 themes by switching between two profiles you set up
  ahead of time. From the iTerm2 Settings page, create a pair of profiles and
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
  [iterm2]
  dark = "my dark theme"
  light = "An Imported Light Theme"
---
