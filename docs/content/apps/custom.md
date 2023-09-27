---
title: Custom Apps
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: other

config_section: x.*
needs_config: true
setup: |
  The `x.` section exists to support custom integrations that aren't yet
  natively supported. Anything that can be achieved with a shell script
  can be done here, including calling other full binaries.

  A few ideas:
  * Use `ssh user@host.example.com thcon dark` to switch themes on a remote
    machine
  * Change your desktop background
  * Turn off your lights with a smart home API call via `curl`
  * Move a custom userchrome.css into place to reskin Firefox
options:
  - key: name
    default: (none)
    description: A human-friendly name for this app, used for logging.
  - key: dark
    default: (none)
    description: |
      The command to run to enter dark mode for the app. Executed as the
      argument to `sh -c`.
  - key: light
    default: (none)
    description: |
      The command to run to enter light mode for the app. Executed as the
      argument to `sh -c`.
example: |
  # Switch vim themes on my remote dev machine
  [x.remote_dev_box]
  name = "vim on dev box"
  dark = "ssh me@example.com thcon dark vim"
  light = "ssh me@example.com thcon light vim"

  [x.wallpaper]
  name = "desktop background"
  dark = "~/bin/set-background.sh '~/Pictures/Dark and Moody.jpg'"
  light = "~/bin/set-background.sh '~/Pictures/Light and Airy.jpg'"
---
