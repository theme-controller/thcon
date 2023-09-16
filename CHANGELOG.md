# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Also I copied this intro verbatim from [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]
### Added:
* [Alacritty](https://alacritty.org) support
* [Changelog](https://thcon.app/changelog) page in HTML docs

### Removed:
* Default dark-mode/light-mode config paths for kitty

## [v0.15.5 - 2023-08-31]
### Fixed:
* No longer reports errors when calling lookandfeeltool on systems without KDE installed.
* No longer reports errors when switching helix themes where a ~/.config/helix directory doesn't exist.

### Changed:
* Skipped apps now use a separate "reason" field when info-level logging (-v) is enabled, instead of listing the skip-reason in the message itself.

## [v0.15.4 - 2023-08-11]
### Fixed:
* Align version number with released version number

## [v0.15.3 - 2023-08-11]
Maintenance release to improve CI infrastructure. No other changes beyond v0.15.1.

## [v0.15.2 - 2023-08-11]
Maintenance release to improve CI infrastructure. No other changes beyond v0.15.1.

## [v0.15.1 - 2023-08-11]
### Fixed:
* When not already running, Terminal.app no longer stays running in the background after switching themes

## [v0.15.0 - 2023-08-11]
Relicensed to BSD 3-clause (previous releases were Apache 2.0 / MIT dual-licensed).

### Changed:
* Updated third-party dependencies

## [v0.14.0 - 2023-06-11]
Completely rewritten in Go. Configuration format is backwards-compatible with
the rust version, but the CLI makes **no claims of backwards compatibility**.

### Added:
* [Helix Editor](https://helix-editor.com) support
* [Kitty](https://sw.kovidgoyal.net/kitty/) support
* Custom application support
* FreeBSD support
* Rewrote docs site, now hosted on https://thcon.app
* `thcon listen` subcommand

### Removed:
* [Sublime Text](https://www.sublimetext.com) support
  * ST4 has been out for long enough that maintaining support for ST3 is unnecessary
* [Visual Studio Code](https://code.visualstudio.com) support
  * VSCode supports OS theme syncing on all major platforms since no later than March 2022
* [Atom](https://github.com/atom/atom) support
  * Atom was sunset in December 2022
* The `thcon-listen` binary is no longer included during installation
  * Use the `thcon listen` subcommand instead

## [v0.13.2 - 2021-11-06]
### Changed:
* Sublime Text is now disabled by default, since ST4 is able to sync with system dark-mode

## [v0.13.1 - 2021-11-05]
### Fixed:
* Systems without a VSCode `settings.json` no longer report errors when switching themes

## [v0.13.0 - 2021-09-18]
### Added:
* Per-application timing is printed when run with `-vvv` (trace-level verbosity)
* Web-based documentation hosted at https://thcon.vercel.app

### Fixed:
* No longer requires `thcon.toml` to exist on-disk for `thcon` to run

### Changed:
* `dark` and `light` are now positional arguments instead of subcommands, which better represents how `thcon` should be
  used.
* Verbose output (`-v` through `-vvv`) is a bit more pretty

## [v0.12.0 - 2021-06-29]
### Added:
* Terminal.app profile switching

### Fixed:
* Documented `disabled` property in app schemas (supported since v0.10.0)
* Corrected documented KDE Konsole section name in `thcon.toml`.  `konsole` was always supported, but the documentation
  was wrong

## [v0.11.0 - 2021-06-28]
### Added:
* GTK theme switching
* GNOME Shell user theme switching

## [v0.10.0 - 2021-06-20]
### Added:
* Allow apps to be disabled with `disabled = true` in their config section
* Establish default values for Atom, KDE Plasma, Sublime Text, and Visual Studio Code

## [v0.9.0 - 2021-03-30]
### Added:
* Support for [Atom](https://atom.io) via an [app-side plugin](https://github.com/theme-controller/thcon-atom)
* Writes vim/nvim rc file to disk so new instances use previously-applied settings.  Requires
  [thcon.vim](https://github.com/theme-controller/thcon.vim) v0.4.0 (at least commit 39b6d82 (v0.4.0, 2021-03-24))

### Removed
* Build-time dependency on `indicatif` crate

## [v0.8.0 - 2021-03-07]
### Added:
* New `--no-replace` CLI arg to `thcon-listen`, to exit cleanly when pre-existing sockets are detected

### Changed:
* Moved `src/main.rs` to `src/bin/thcon.rs` to better align with `thcon-listen` binary

## [v0.7.0 - 2021-03-06]
### Added:
* New `thcon-listen` binary for use with app-specific plugins

### Changed:
* vim: Use `thcon-listen` for IPC transport
* iterm2: Use `thcon-listen` for IPC transport

## [v0.6.0] - 2021-02-12
### Added:
* Support for changing [Sublime Text 3](https://sublimetext.com) color schemes and themes

## [v0.5.1] - 2021-02-10
### Removed:
* Removed progress bar with and without `--verbose`.  `thcon` tends to run fast enough to not warrant a progress bar.

### Fixed:
* No longer panics when `thcon.toml` doesn't exist or is invalid TOML
* No longer panics when no or invalid subcommand provided at commandline

### Changed:
* Moved `help` subcommand to the bottom of subcommand list in `--help`

## [v0.5.0] - 2021-02-08
### Added:
* Support for changing [iTerm2](https://iterm2.com) profiles

### Changed:
* Updated dependencies

## [v0.4.3] - 2021-01-23
### Fixed:
* VSCode: preserve comma in settings.json if one is present
* Silence unused import during build on non-mac platforms

## [v0.4.2] - 2021-01-22
### Changed:
* macOS: `thcon` now reads its config file from ~/.config/thcon/thcon.toml instead of ~/Library/Application Support/thcon/thcon.toml
* macOS: Support `thcon.vim` 0.2.0 (named pipes are written to ~/.local/share/thcon/ instead of ~/Library/Application Support/thcon/)

## [v0.4.1] - 2021-01-19
### Fixed:
* `thcon` can once again be successfully installed on macOS

## [v0.4.0] - 2021-01-18
### Added:
* `--verbose`/`-v` argument to enable verbose logging of re-themed and skipped
  (unconfigured) applications
* Configuration documentation for KDE Plasma
* Configuration documentation for KDE Konsole

### Fixed:
* Stop logging errors for unconfigured applications (silently skips unconfigured
  apps instead)
* Don't require apps to be listed individually on CLI (try all apps by default
  with `thcon dark` / `thcon light`)
* Suppress spurious error messages when switching `plasma` themes
* Explicitly listed `thcon.toml` section to use for `vim`/`nvim`
* Lots of lint failures

### Changed:
* The `plasma` app (KDE Plasma) is now only available on non-macOS, non-Windows
  platforms

## [v0.3.0] - 2021-01-18

### Removed:
* `Themeable::toggle()`, which was previously used to implicitly switch
  from light to dark (or vice-versa, depending on the current state) has
  been removed.  It may return in a future release, but improving app
  support is more important right now.

### Added:
* `vim` & `neovim` theme switching via [thcon.vim](https://github.com/sjbarag/thcon.vim)
* [This changelog](./CHANGELOG.md)

### Changed:
* Started documenting per-app configuration direction in crates to
  enable serving via https://docs.rs


## [v0.2.0] - 2020-10-27
    
### Added:
* macOS global theme switching
* alacritty color switching
* First set of per-app documentation (may not be the correct place
  for those to live long-term though?)
    
### Fixes:
* Allow installation on systems that don't support DBus
* Don't crash if ~/.config/thcon/thcon.toml doesn't exist (helpful on
  macOS, where there's no configuration required to switch between light
  and dark modes)
