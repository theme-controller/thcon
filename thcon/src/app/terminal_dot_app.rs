//! Switches between [Terminal.app
//! profiles](https://support.apple.com/guide/terminal/profiles-change-terminal-windows-trml107/mac).
//!
//! ## Usage: macOS
//! Terminal.app's default "Basic" profile has is aware of the macOS dark mode setting, and will
//! react accordingly.  No other themes appear to behave that way, however.  For manually-added (or
//! imported) profiles, simply list the names of the desired light mode and dark mode profiles in
//! your `thcon.toml`.
//!
//! ```toml
//! [terminal-app]
//! dark = "Pro"
//! light = "Silver Aerogel"
//! ```
//!
//! ## Usage: Windows, Linux & BSD
//! Apple's Terminal.app is only available on macOS, so this module is only usable on macOS as a result.
//!
//! ## `thcon.toml` Schema
//! Section: `terminal-app`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
//! | `dark` | string | The name of the profile to use in dark mode | Pro |
//! | `light` | string | The name of the profile to use in light mode | Basic |

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use crate::themeable::{ConfigState, Themeable};
use crate::AppConfig;
use crate::Disableable;

use std::error::Error;
use std::process::Command;

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
            light: "Basic".to_string(),
            dark: "Pro".to_string(),
            disabled: false,
        }
    }
}

pub struct TerminalDotApp;

impl Themeable for TerminalDotApp {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_default_config(config.terminal_dot_app.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let default_config = _Config::default();

        let config = match self.config_state(config) {
            ConfigState::NoDefault => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Default => &default_config,
            ConfigState::Enabled => config
                .terminal_dot_app
                .as_ref()
                .unwrap()
                .unwrap_inner_left(),
        };

        let profile_name = match operation {
            Operation::Lighten => &config.light,
            Operation::Darken => &config.dark,
        };

        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "tell application \"Terminal\"
                      set new_settings to first settings set whose name is \"{}\"

                      set default settings to new_settings
                      set startup settings to new_settings

                      set current settings of every tab of every window to new_settings
                    end tell",
                profile_name
            ))
            .status()
            .expect("Failed to execute `osascript`");

        Ok(())
    }
}
