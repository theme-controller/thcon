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
            ConfigState::Enabled => config.terminal_dot_app.as_ref().unwrap().unwrap_inner_left(),
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
            )).status()
            .expect("Failed to execute `osascript`");

        Ok(())
    }
}
