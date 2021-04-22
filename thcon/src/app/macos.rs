//! Switches between Light and Dark [appearances](https://support.apple.com/en-us/HT208976) in macOS.
//!
//! ## Usage
//! There's no configuration required!  `thcon dark` will always enable dark mode on macOS, and
//! `thcon light` will disable it, but it _can_ be disabled with `disabled = true`.

use crate::themeable::{ConfigState, Themeable};
use crate::operation::Operation;
use crate::config::Config as ThconConfig;
use crate::Disableable;
use crate::AppConfig;

use std::error::Error;
use std::process::Command;

use serde::Deserialize;

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    #[serde(default)]
    disabled: bool,
}

impl Default for _Config {
    fn default() -> Self {
        Self { disabled: false }
    }
}

pub struct MacOS;

impl Themeable for MacOS {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_default_config(either!(()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
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
            )).status()
            .expect("Failed to execute `osascript`");

        Ok(())
    }
}
