//! Switches between [KDE Plasma](https://kde.org/plasma-desktop/) theme packages
//!
//! ## Usage: Linux & BSD
//! KDE Plasma already ships with a commandline tool to switch global UI themes: `lookandfeeltool`.
//! `thcon` simply shells out to that command, so configuring `thcon` requires a brief interaction
//! with it.
//!
//! Run `lookandfeeltool --list` to show all available theme packages. Choose the theme packages you
//! want for light and dark mode, then list those in your `thcon.toml`, e.g.:
//!
//! ```toml
//! [plasma]
//! dark = "org.kde.breezedark.desktop"   # the default dark theme
//! light = "org.kde.breeze.desktop"      # the default light theme
//! ```
//!
//! ## Usage: Windows & macOS
//! KDE Plasma is not supported on these platforms.
//!
//! ## `thcon.toml` Schema
//! Section: `plasma`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | `dark` | string | The theme package name to use in dark mode | (none) |
//! | `light` | string | The theme package name to use in light mode | (none) |

use crate::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

use std::error::Error;
use std::io;
use std::process::{Command,Stdio};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    light: String,
    dark: String,
}

pub struct Plasma;

impl Themeable for Plasma {
    fn has_config(&self, config: &ThconConfig) -> bool {
        config.plasma.is_some()
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.plasma {
            Some(plasma) => plasma,
            None => {
                return Err(
                    Box::new(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            "Couldn't find [plasma] section in thcon.toml"
                        )
                    )
                );
            }
        };

        let theme = match operation {
            Operation::Lighten => &config.light,
            Operation::Darken => &config.dark,
        };

        Command::new("lookandfeeltool")
            .stderr(Stdio::null())
            .arg("--apply")
            .arg(theme)
            .status()
            .expect("Failed to execute `lookandfeeltool`");

        Ok(())
    }
}
