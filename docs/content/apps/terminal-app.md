---
title: Terminal.app
layout: layouts/app.html
platforms: [ macos ]
category: terminal

config_section: terminal-app
needs_config: false
url: https://support.apple.com/guide/terminal/welcome/mac
setup: |
  Terminal.app's default "Basic" profile is aware of the
  [system dark mode](/apps/macos) and reacts when it changes, but no other
  themes appear to behave that way.

  For all other profiles, `thcon` controls Terminal.app themes by switching
  between two profiles you set up ahead of time. From the Terminal.app Settings
  page, create (or import) a pair of profiles and configure them to your
  liking. Use the names of those profiles in `thcon.toml`
options:
  - key: dark
    default: Pro
    description: Profile name to use in dark mode
  - key: light
    default: Basic
    description: Profile name to use in light mode
example: |
  [terminal-app]
  dark = "Red Sands"
  light = "Man Page"
---
