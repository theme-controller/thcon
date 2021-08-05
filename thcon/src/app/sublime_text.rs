//! Switches between [Sublime Text 3](https://sublimetext.com) themes and color schemes
//!
//! ## Aside: Comments and sublime-settings order
//! Changing through the Sublime Text UI any setting that can appear in
//! `Preferences.sublime-settings` causes that file to be completely rewritten. This causes `// ...`
//! comments to be completely removed, and results in keys that are sorted alphabetically. `thcon`
//! matches this behavior.
//!
//! ## Usage
//! Sublime Text monitors its `Preferences.sublime-settings` file for changes while it's running,
//! applying changes as they appear. `thcon` will parse that file, replace the `theme` and
//! `color_scheme` values (if values are provided in `thcon.toml`), and write the new file back
//! in-place. Copy the `color_scheme` and `theme` values from your `Preferences.sublime-settings`
//! into `thcon.toml`:
//!
//! ```toml
//! [sublime-text]
//! # (optional) tell `thcon` where your preferences are if they're not in the default location
//! # preferences = /path/to/your/Preferences.sublime-settings
//!
//! [sublime-text.dark]
//! color_scheme = "Packages/Color Scheme - Default/Monokai.sublime-color-scheme"
//! theme = "Default.sublime-theme"
//!
//! [sublime-text.light]
//! color_scheme = "Packages/Color Scheme - Default/Celeste.sublime-color-scheme"
//! theme = "Adaptive.sublime-theme"
//! ```
//!
//! ## `thcon.toml` Schema
//! Section: `sublime-text`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `true` |
//! | `light` | table | Settings to apply in light mode | |
//! | `light.color_scheme` | string | The `color_scheme` to use in light mode | `Packages/Color Scheme - Default/Celeste.sublime-color-scheme` |
//! | `light.theme` | string | The `theme` to use in light mode | `Adaptive.sublime-theme` |
//! | `dark` | table | Settings to apply in dark mode | |
//! | `light.color_scheme` | string | The `color_scheme` to use in dark mode | `Packages/Color Scheme - Default/Monokai.sublime-color-scheme` |
//! | `light.theme` | string | The `theme` to use in dark mode | `Default.sublime-theme` |
//! | `preferences` | string | Absolute path to your `Preferences.sublime-settings` file | Default Sublime Text 3 locations: <ul><li>Linux/BSD: `~/.config/sublime-text-3/Packages/User/Preferences.sublime-settings`</li><li>macOS: `~/Library/Application Support/Sublime Text 3/Packages/User/Preferences.sublime-settings`</li></ul> |

use std::error::Error;
use std::fs::{self,OpenOptions};
use std::path::PathBuf;

use crate::themeable::{ConfigState, Themeable};
use crate::operation::Operation;
use crate::config::Config as ThconConfig;
use crate::Disableable;
use crate::AppConfig;

use log::{debug, warn};
use serde::{Serialize,Deserialize};
use serde_json::ser::{PrettyFormatter, Serializer};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    light: ConfigSection,
    dark: ConfigSection,
    #[serde(rename = "preferences")]
    preferences_file: Option<String>,
    #[serde(default)]
    disabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigSection {
    color_scheme: Option<String>,
    theme: Option<String>,
}

impl Default for _Config {
    fn default() -> Self {
        Self {
            light: ConfigSection {
                color_scheme: Some("Packages/Color Scheme - Default/Celeste.sublime-color-scheme".to_string()),
                theme: Some("Adaptive.sublime-theme".to_string()),
            },
            dark: ConfigSection {
                color_scheme: Some("Packages/Color Scheme - Default/Monokai.sublime-color-scheme".to_string()),
                theme: Some("Default.sublime-theme".to_string()),
            },
            preferences_file: None,
            disabled: false,
        }
    }
}

fn preferences_path() -> PathBuf {
    [
        dirs::config_dir().unwrap().to_str().unwrap(),
        #[cfg(mac)]
        "Sublime Text 3",
        #[cfg(not(mac))]
        "sublime-text-3",
        "Packages",
        "User",
        "Preferences.sublime-settings"
    ].iter().collect()
}
pub struct SublimeText;

impl Themeable for SublimeText {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_default_config(config.sublime_text.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let default_config = _Config::default();

        let config = match self.config_state(config) {
            ConfigState::NoDefault => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Default => &default_config,
            ConfigState::Enabled => config.sublime_text.as_ref().unwrap().unwrap_inner_left(),
        };

        let section = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };

        let settings_path = match &config.preferences_file {
            Some(pathstr) => PathBuf::from(pathstr),
            None => preferences_path(),
        };

        debug!(
            "Reading/writing Preferences.sublime-settings at {}",
            &settings_path.display()
        );

        let settings = fs::read_to_string(&settings_path).unwrap_or_default();
        let mut settings: JsonValue = serde_json::from_str(&settings).unwrap_or_default();
        if let Some(color_scheme) = &section.color_scheme {
            settings["color_scheme"] = JsonValue::String(color_scheme.to_string());
        }
        if let Some(theme) = &section.theme {
            settings["theme"] = JsonValue::String(theme.to_string());
        }

        let maybe_settings_file = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .open(&settings_path);
        if let Ok(file) = maybe_settings_file {
            // sublime-text uses four-space indents for its Preferences.sublime-settings file
            // so set up a custom formatter and serializer to match that style
            let formatter = PrettyFormatter::with_indent(b"    ");
            let mut serializer = Serializer::with_formatter(file, formatter);
            settings.serialize(&mut serializer).unwrap();
        } else {
            warn!(
                "Could not find Preferences.sublime-settings at {}",
                &settings_path.display()
            );
        }

        Ok(())
    }
}
