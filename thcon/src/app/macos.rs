//! Switches between Light and Dark [appearances](https://support.apple.com/en-us/HT208976) in macOS.
//!
//! ## Usage
//! There's no configuration required!  `thcon dark` will enable dark mode on macOS, and
//! `thcon light` will disable it, but this behavior can be disabled with `disabled = true`.
//!
//! ## `thcon.toml` Schema
//! Section: `macos`
//!
//! | Key | Type | Description | Default  |
//! | --- | ---- | ----------- | -------- |
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use crate::themeable::{ConfigState, Themeable};
use crate::AppConfig;
use crate::Disableable;

use std::process::Command;

use anyhow::anyhow;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    #[serde(default)]
    disabled: bool,
}

pub struct MacOS;

impl Themeable for MacOS {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_default_config(config.macos.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()> {
        match self.config_state(config) {
            ConfigState::NoDefault => unreachable!(),
            ConfigState::Default => (),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Enabled => (),
        };

        let dark_mode = match operation {
            Operation::Lighten => false,
            Operation::Darken => true,
        };

        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "tell app \"System Events\" to \
                     tell appearance preferences to \
                     set dark mode to {}",
                dark_mode
            ))
            .status()
            .context("Failed to execute 'osascript'")
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(anyhow!("Failed to execute 'osascript'"))
                }
            })
    }
}
