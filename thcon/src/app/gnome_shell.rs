//! Switches between [GNOME Shell](https://wiki.gnome.org/Projects/GnomeShell) user themes, like
//! the [User Themes extension](https://extensions.gnome.org/extension/19/user-themes/) does
//!
//! ## Usage: Linux & BSD
//! GNOME Shell user themes require the [User Themes
//! extension](https://extensions.gnome.org/extension/19/user-themes/) to be enabled.  Once that's
//! done, simply provide the name of the theme as displayed in the User Themes extension config
//! (either via GNOME Extensions or GNOME Tweaks), e.g.:
//!
//! ```toml
//! [gnome-shell]
//! light = "Arc"
//! dark = "Arc-Dark-solid"
//! ```
//!
//! ## Usage: Windows & macOS
//! Currently unsupported.
//!
//! ## `thcon.toml` Schema
//! Section: `gnome-shell`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | ------- |
//! | `dark` | string | The name of the theme (case-sensitive) to apply in dark mode | (none) |
//! | `light` | string | The name of the theme (case-sensitive) to apply in light mode | (none) |
//!

use std::error::Error;

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use crate::themeable::{ConfigError, ConfigState, Themeable};
use crate::AppConfig;
use crate::Disableable;

use gio::SettingsExt;
use log::error;
use serde::Deserialize;

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    light: String,
    dark: String,
    #[serde(default)]
    disabled: bool,
}

pub struct GnomeShell {}

impl Themeable for GnomeShell {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_manual_config(config.gnome_shell.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match self.config_state(config) {
            ConfigState::NoDefault => {
                return Err(Box::from(ConfigError::RequiresManualConfig("gnome_shell")))
            }
            ConfigState::Default => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Enabled => config.gnome_shell.as_ref().unwrap().unwrap_inner_left(),
        };

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };

        let gsettings = gio::Settings::new("org.gnome.shell.extensions.user-theme");
        match gsettings.set_string("name", &theme) {
            Ok(_) => gio::Settings::sync(),
            Err(e) => error!("Unable to apply GNOME Shell user theme: {}", e),
        };

        Ok(())
    }
}
