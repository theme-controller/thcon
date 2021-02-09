# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Also I copied this intro verbatim from [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### Removed:
* Removed progress bar from output, since this application usually takes < 0.5 seconds to complete.

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
