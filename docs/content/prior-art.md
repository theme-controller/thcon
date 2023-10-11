---
title: Prior Art
layout: layouts/main.html
eleventyNavigation:
  parent: root
  key: install
  title: install
---

# Prior Art
This isn't the first theme switcher, and it certainly won't be the last.
Alternative tools are worth discussing. They may match your needs better, and
the discussion helps to show how `thcon` fits into the space.

Broadly speaking though: Most of these tools optimize for switching to an
entirely new colorscheme often. For example, switching from
[Solarized](https://ethanschoonover.com/solarized/) to
[Catppuccin](https://github.com/catppuccin) to
[gruvbox](https://github.com/morhetz/gruvbox) to (…). These often exclude system
settings, which can lead to an unexpectedly still-bright file browser or a
system volume slider that's too dark for the current surroundings.

`thcon` instead optimizes for switching between dark and light variants of a
single theme, and considers desktop environment / OS settings "in scope". If you
make your terminal dark, you likely also want your global panel, file navigator,
window switcher, system dialogs etc. to also be dark.

## Gogh
[Gogh](https://github.com/Gogh-Co/Gogh) is an excellent theme installer that
can automatically set up a new profile in one of several terminals. It's a good
complement to `thcon`, since it can reduce the effort involved in setting up
profiles when adopting a new color scheme.

## Base16 + Flavours
The [Base16 project](https://github.com/chriskempson/base16) has produced tons
of themes and spawned several accompanying tools.

### Flavours
[Flavours](https://github.com/Misterio77/flavours) supports far more
applications due to its support for
[base16-templates-source](https://github.com/chriskempson/base16-templates-source),
but it's unable to switch desktop environment themes, integrate with tools that
don't support config-file switching (e.g. macOS's global dark theme), or use
themes that are outside the base16 family.

### base16-universal-manager
[base16-universal-manager](https://github.com/pinpox/base16-universal-manager)
is quite similar to `flavours`, and will download, install, and switch themes
for any [base16-templates-source](https://github.com/chriskempson/base16-templates-source)
app. It unfortunately has the same limitations as `flavours` as a result.

## Pywal
A classic in this space, [pywal](https://github.com/dylanaraps/pywal) is pretty
similar to `flavours`/`base16-universal-manager`. It can control many of the
same tools, and its likely most popular for its ability to generate a custom
colorscheme from an image. Like the other tools listed, it's unable to switch
more global OS-level themes.

## Missing Something?
There's certainly a few tools missing here. Please
[open an issue](https://github.com/theme-controller/thcon/issues/new) and let
me know what I've missed.
