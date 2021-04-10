//! Switches [alacritty](https://github.com/alacritty/alacritty) color schemes
//!
//! Since alacritty is configured via [yaml](https://yaml.org/), using anchors and aliases is the
//! simplest way of managing color schemes.
//! 
//! ## Usage
//! In your `alacritty.yml`, define your colors
//! 
//! ```yaml
//! # define your color themes:
//! 
//! solarized: &solarized_dark
//!   #         ^^^^^^^^^^^^^^ - use this name in thcon.toml
//!   primary:
//!     background: '0x002b36'
//!     foreground: '0x839496'
//!   # ... the normal contents of a `colors` object
//! 
//! light_solarized: &solarized_light:
//!   #               ^^^^^^^^^^^^^^^ - use this name in thcon.toml
//!   primary:
//!     background: '0xfdf6e3'
//!     foreground: '0x586e75'
//! 
//! # then choose your color scheme one last time:
//! colors: *solarized_light # thcon:replace-line
//! 
//! # thcon will manage the line ending in `thcon:replace-line`
//! # to swap alacritty color schemes
//! ```
//! 
//! In your `thcon.toml`, define light and dark themes based on the `&anchor`s defined above:
//! 
//! ```toml
//! [alacritty]
//! dark = "solarized_dark"
//! light = "solarized_light"
//! 
//! # optionally, tell thcon where your alacritty config is stored
//! config = "/path/to/alacritty.yml"
//! ```
//! 
//! ## `thcon.toml` Schema
//! Section: `alacritty`
//! 
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | `dark` | string | The YAML anchor (declared in `alacritty.yml`) used for dark mode | (none) |
//! | `light` | string | The YAML anchor (declared in `alacritty.yml`) used for light mode | (none) |
//! | `config` | string | Absolute path to your `alacritty.yml` file | (see below) |
//! 
//! ### Default value for `config`
//! Thcon checks all default locations that `alacritty` [defines for alacritty.yml](https://github.com/alacritty/alacritty#configuration):
//! 
//! * Windows: `%APPDATA%\alacritty\alacritty.yml`
//! * Other platforms:
//!   1. `$XDG_CONFIG_HOME/alacritty/alacritty.yml`
//!   2. `$XDG_CONFIG_HOME/alacritty.yml`
//!   3. `$HOME/.config/alacritty/alacritty.yml`
//!   4. `$HOME/.alacritty.yml`

use std::error::Error;
use std::fs;
use std::env;
use std::path::PathBuf;

use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

use log::{error, debug};
use serde::Deserialize;
use regex::{Captures,Regex};

#[derive(Debug, Deserialize)]
pub struct Config {
    light: String,
    dark: String,
    config: Option<String>,
}

pub struct Alacritty;

impl Themeable for Alacritty {
    fn has_config(&self, config: &ThconConfig) -> bool {
        config.alacritty.is_some()
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.alacritty {
            Some(alacritty) => alacritty,
            None => {
                error!("Couldn't find [alacritty] section in thcon.toml");
                return Ok(());
            }
        };

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };

        let alacritty_yaml = match alacritty_config() {
            Some(path) => path,
            None => {
                let couldnt_find = String::from("Couldn't find alacritty.yml");
                let message = match &config.config {
                    Some(_path) => couldnt_find,
                    None => format!("{}; consider adding `config` property to `[alacritty]` section of `thcon.toml`", couldnt_find),
                };

                error!("{}", message);
                return Ok(());
            }
        };

        debug!("Reading/writing alacritty.yml at {}", alacritty_yaml.display());

        match fs::read_to_string(&alacritty_yaml) {
            Ok(settings) => {
                let theme_regex = Regex::new(r#"^(?P<prefix>"?colors"?\s*:\s*"?)(?P<v>[^\s]+)(?P<suffix>"?,?\s*#\s*thcon:replace-line)"#)?;
                let modified_lines: Vec<String> = settings.lines().map(|line| {
                    theme_regex.replace(line, |caps: &Captures| {
                        format!("{}*{}{}", &caps["prefix"], theme, &caps["suffix"])
                    }).into_owned()
                }).collect();
                let settings = modified_lines.join("\n");

                fs::write(&alacritty_yaml, settings).map_err(|err| {
                    Box::new(err) as Box<dyn Error>
                })
            },
            Err(e) => {
                error!("Unable to read settings: {}", e);
                Err(Box::new(e))
            }
        }
    }
}

// copied from upstream `alacritty`:
// https://github.com/alacritty/alacritty/blob/5a3bf69e3fd771271921f62219cdb8f920db39ee/alacritty/src/config/mod.rs#L236-L274
/// Get the location of the first found default config file paths
/// according to the following order:
///
/// 1. $XDG_CONFIG_HOME/alacritty/alacritty.yml
/// 2. $XDG_CONFIG_HOME/alacritty.yml
/// 3. $HOME/.config/alacritty/alacritty.yml
/// 4. $HOME/.alacritty.yml
#[cfg(not(windows))]
fn alacritty_config() -> Option<PathBuf> {
    // Try using XDG location by default.
    xdg::BaseDirectories::with_prefix("alacritty")
        .ok()
        .and_then(|xdg| xdg.find_config_file("alacritty.yml"))
        .or_else(|| {
            xdg::BaseDirectories::new()
                .ok()
                .and_then(|fallback| fallback.find_config_file("alacritty.yml"))
        })
        .or_else(|| {
            if let Ok(home) = env::var("HOME") {
                // Fallback path: $HOME/.config/alacritty/alacritty.yml.
                let fallback = PathBuf::from(&home).join(".config/alacritty/alacritty.yml");
                if fallback.exists() {
                    return Some(fallback);
                }
                // Fallback path: $HOME/.alacritty.yml.
                let fallback = PathBuf::from(&home).join(".alacritty.yml");
                if fallback.exists() {
                    return Some(fallback);
                }
            }
            None
        })
}

// copied from upstream `alacritty`:
// https://github.com/alacritty/alacritty/blob/5a3bf69e3fd771271921f62219cdb8f920db39ee/alacritty/src/config/mod.rs#L236-L274
#[cfg(windows)]
fn alacritty_config() -> Option<PathBuf> {
    dirs::config_dir().map(|path| path.join("alacritty\\alacritty.yml")).filter(|new| new.exists())
}
