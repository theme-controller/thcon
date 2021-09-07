//! Switches between [GTK](https://www.gtk.org/) application themes
//!
//! ## Usage: Linux & BSD
//! The active GTK application theme is managed by
//! [dconf](https://developer.gnome.org/dconf/unstable/dconf-overview.html), and typically accessed
//! as a user via something like [Gnome Tweaks](https://wiki.gnome.org/Apps/Tweaks) or [KDE GTK
//! Configurator](https://invent.kde.org/plasma/kde-gtk-config).  `thcon` can include GTK theme
//! switching quite simply by reading the desired theme names from `thcon.toml`, e.g.:
//!
//! ```toml
//! [gtk]
//! dark = HighContrastInverse
//! light = HighContrast
//! ```
//!
//! The value should be the name of the desired theme as reported in Gnome Tweaks, or its filename
//! in `/usr/themes/`, `/usr/local/themes/` or `~/.themes/`.  Invalid values default to `Adwaita`,
//! with `gnome-terminal` ignoring any preferences to default to UI controls.
//!
//! ## Usage: Windows & macOS
//! Currently unsupported.
//!
//! ## `thcon.toml` Schema
//! Section: `gtk`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | ------- |
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
//! | `dark` | string | The name of the theme (case-sensitive) to apply in dark mode | `Adwaita-dark` |
//! | `light` | string | The name of the theme (case-sensitive) to apply in light mode | `Adwaita` |
//!

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use crate::themeable::{ConfigState, Themeable};
use crate::AppConfig;
use crate::Disableable;

use anyhow::{Context, Result};
use gio::SettingsExt;
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
            light: "Adwaita".to_string(),
            dark: "Adwaita-dark".to_string(),
            disabled: false,
        }
    }
}

pub struct Gtk {}

impl Themeable for Gtk {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_default_config(config.gtk.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()> {
        let default_config = _Config::default();

        let config = match self.config_state(config) {
            ConfigState::NoDefault => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Default => &default_config,
            ConfigState::Enabled => config.gtk.as_ref().unwrap().unwrap_inner_left(),
        };

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };

        let gsettings = gio::Settings::new("org.gnome.desktop.interface");
        gsettings
            .set_string("gtk-theme", theme)
            .map(|_| gio::Settings::sync())
            .with_context(|| format!("Unable to apply GTK theme '{}'", theme))
    }
}
