---
title: Usage
layout: layouts/main.html
eleventyNavigation:
  parent: root
  key: usage
  title: usage
---

# Usage
`thcon` usage happens in two phases. In the first, you configure your
applications and write your `config.toml`. In the second, you use the `thcon`
command line to make things dark/light and get back to what you were doing.

## Configure Apps
Consult the [app support table](https://thcon.app/apps/) and configure the
applications you want to switch themes in using the included documentation.

## Use CLI
To switch all configured apps to dark mode, simply run:

```text
thcon dark
```

To go back to light mode:

```text
thcon light
```

To switch just some apps, list them after `dark` or `light`:

```text
# Lighten helix and iterm2, and macOS only.
thcon light helix iterm2 macos
```

To view a list of apps supported on your platform, use `--help`:

```text
$ thcon dark --help
Switches to apps to dark mode

Usage:
  thcon dark [app...] [flags]

Apps:
  alacritty, emacs, helix, iterm2, kitty, macos, neovim, terminal-app, vim

Flags:
  -h, --help            help for dark
  -v, --verbose count   enable verbose logging (add multiple times for higher verbosity)
```
