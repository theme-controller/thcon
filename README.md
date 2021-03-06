# thcon
Switches multiple apps between light and dark mode

## Goal
Sometimes it's bright.  Sometime's it's dark.  Switching between light and dark themes in your terminals, your editor(s), your desktop environment, and all your other apps is a pain.  `thcon` aims to be a near universal theme controller driven by a single config file, letting you switch themes in all your apps with one command.

## Installation
For all platforms, please install Rust following [the recommended
instructions](https://www.rust-lang.org/learn/get-started).

### Ubuntu 20.04 (Focal)
```
sudo apt install build-essential libdbus-1-dev libglib2.0-dev
cargo install thcon
```

### Fedora 34
```
sudo dnf install dbus-devel glib2-devel
cargo install thcon
```

### openSUSE Leap (15.3)
```
sudo zypper install gcc dbus-1-devel glib2-devel
cargo install thcon
```

## Naming
`thcon` is short for "THeme CONtroller", pronounced like [the English word "thicken"](https://en.wiktionary.org/wiki/thicken).  The name was chosen from a misunderstanding of [pkcon](http://manpages.ubuntu.com/manpages/trusty/man1/pkcon.1.html) as "PacKage CONtroller", and was maintained despite `pkcon` actually being a "PackageKit CONsole client".

Feel free to pretend this less embarrassing explanation is the truth:

> `thcon` (pronounced like [the English word "thicken"](https://en.wiktionary.org/wiki/thicken)) comes from the use of a [roux to thicken a sauce](https://en.wikipedia.org/wiki/Roux) while cooking, and is a play on the fact that a roux can cooked to either a light or dark color.  A roux thickens your sauce and can be light or dark; `thcon` makes your apps light or dark.  Perhaps it should have been called `roux`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
