---
title: Install
layout: layouts/main.html
eleventyNavigation:
  parent: root
  key: install
  title: install
---

# Install
Currently, the simplest way to install `thcon` is with the `go` CLI:

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

# Download
Static binaries will be available via the
[GitHub Releases page](https://github.com/theme-controller/thcon/releases) when
this project hits 1.0.