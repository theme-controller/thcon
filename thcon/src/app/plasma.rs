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
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
//! | `dark` | string | The theme package name to use in dark mode | `org.kde.breezedark.desktop` |
//! | `light` | string | The theme package name to use in light mode | `org.kde.breeze.desktop` |

use crate::{Themeable, themeable::ConfigState};
use crate::operation::Operation;
use crate::config::Config as ThconConfig;
use crate::Disableable;
use crate::AppConfig;

use std::process::{Command,Stdio};

use anyhow::anyhow;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    light: String,
    dark: String,
    #[serde(default)]
    disabled: bool,
}

impl Default for _Config {
    fn default() -> Self {
        Self {
            light: "org.kde.breeze.desktop".to_string(),
            dark: "org.kde.breezedark.desktop".to_string(),
            disabled: false
        }
    }
}

pub struct Plasma;

impl Themeable for Plasma {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_default_config(config.plasma.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()> {
        let default_config = _Config::default();

        let config = match self.config_state(config) {
            ConfigState::NoDefault => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Default => &default_config,
            ConfigState::Enabled => config.plasma.as_ref().unwrap().unwrap_inner_left(),
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
            .with_context(|| format!("Failed to execute 'lookandfeeltool --apply {}'", theme))
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(anyhow!("Failed to execute 'lookandfeeltool --apply {}'", theme))
                }
            })
    }
}
