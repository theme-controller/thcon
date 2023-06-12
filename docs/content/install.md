---
title: Install
layout: layouts/main.html
eleventyNavigation:
  parent: root
  key: install
  title: install
---

# Install
## Pre-built Binaries
Static binaries for each supported platform/architecture pair are attached to
each release on GitHub:

[thcon Releases](https://github.com/theme-controller/thcon/releases)

## go install
You can also build and install `thcon` from-source using the `go` CLI:

```sh
go install github.com/theme-controller/thcon@latest
```

This, naturally, requires a Go toolchain. Linux and FreeBSD systems require a
native dependency or two, but they may already be installed.

## Dependencies
### Ubuntu
```sh
apt install libglib2.0-dev
```

### Fedora
```sh
dnf install glib2-devel
```

### FreeBSD
```sh
pkg install glib pkgconf
```

## OS Packages
`thcon` isn't currently packaged for any operating systems. If you're willing to
help with packaging, please [file a GitHub Issue](https://github.com/theme-controller/thcon/new)
to let me know!
