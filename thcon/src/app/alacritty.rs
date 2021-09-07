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
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
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

use std::fs;
use std::path::PathBuf;

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use crate::themeable::{ConfigError, ConfigState, Themeable};
use crate::AppConfig;
use crate::Disableable;

use anyhow::{Context, Result};
use log::{debug, error};
use regex::{Captures, Regex};
use serde::Deserialize;

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    light: String,
    dark: String,
    config: Option<String>,
    #[serde(default)]
    disabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Alacritty {}

impl Themeable for Alacritty {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_manual_config(config.alacritty.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()> {
        let config = match self.config_state(config) {
            ConfigState::NoDefault => {
                return Err(ConfigError::RequiresManualConfig("alacritty").into())
            }
            ConfigState::Default => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Enabled => config.alacritty.as_ref().unwrap().unwrap_inner_left(),
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

        debug!(
            "Reading/writing alacritty.yml at {}",
            alacritty_yaml.display()
        );

        match fs::read_to_string(&alacritty_yaml)
            .with_context(|| format!("Unable to read settings from {}", &alacritty_yaml.display()))
        {
            Ok(settings) => {
                let theme_regex = Regex::new(
                    r#"^(?P<prefix>"?colors"?\s*:\s*"?)(?P<v>[^\s]+)(?P<suffix>"?,?\s*#\s*thcon:replace-line)"#,
                )?;
                let modified_lines: Vec<String> = settings
                    .lines()
                    .map(|line| {
                        theme_regex
                            .replace(line, |caps: &Captures| {
                                format!("{}*{}{}", &caps["prefix"], theme, &caps["suffix"])
                            })
                            .into_owned()
                    })
                    .collect();
                let settings = modified_lines.join("\n");

                fs::write(&alacritty_yaml, settings).with_context(|| {
                    format!("Unable to write settings to {}", &alacritty_yaml.display())
                })
            }
            Err(e) => {
                error!("Unable to read settings: {}", e);
                Err(e)
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
    use std::env;

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
    dirs::config_dir()
        .map(|path| path.join("alacritty\\alacritty.yml"))
        .filter(|new| new.exists())
}
