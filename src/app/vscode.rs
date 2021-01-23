//! Switches between [Visual Studio Code](https://code.visualstudio.com/) themes
//!
//! ## Usage: Windows & macOS
//! Since [version
//! 1.42](https://code.visualstudio.com/updates/v1_42#_auto-switch-theme-based-on-os-color-scheme),
//! Visual Studio Code can listen to Windows and macOS color scheme changes and switch to a
//! matching theme.  It's recommended for use on those platforms.
//!
//! ## Usage: Linux & BSD
//! Visual Studio Code doesn't support OS synchronization on Linux and non-Apple BSDs, so `thcon`
//! is the recommended method.  Visual Studio Code monitors its `settings.json` file for changes
//! while it's running.  Because that `settings.json` file can include comments, the simplest way
//! to preserve existing whitespace and comments is by looking for a magic comment annotating the
//! `workbench.colorTheme` setting.
//!
//! In your `settings.json`, mark the `workspace.colorTheme` line so `thcon` can find it:
//!
//! ```jsonc
//! {
//!   // ... other settings
//!
//!   "workbench.colorTheme": "" // thcon:replace-line
//! }
//! ```
//!
//! In your `thcon.toml`, define light and dark themes:
//!
//! ```toml
//! [vscode]
//! dark = "Default Dark+"   # the default dark theme
//! light = "Default Light+" # the default light theme
//!
//! # optionally, tell thcon where your settings.json is stored
//! config = "/path/to/settings.json"
//! ```
//!
//! ## `thcon.toml` Schema
//! Section: `vscode`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | `dark` | string | The theme to use in dark mode | (none) |
//! | `light` | string | The theme to use in light mode | (none) |
//! | `config` | string | Absolute path to your `settings.json` file | `~/.config/Code/User/settings.json` |

use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use regex::{Captures,Regex};

use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    light: String,
    dark: String,
    config: Option<String>,
}

pub struct VSCode;

impl VSCode {
    fn settings_json_path(&self) -> PathBuf {
        [
            dirs::config_dir().unwrap().to_str().unwrap(),
            "Code",
            "User",
            "settings.json"
        ].iter().collect()
    }
}

impl Themeable for VSCode {
    fn has_config(&self, config: &ThconConfig) -> bool {
        config.vscode.is_some()
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.vscode {
            Some(vscode) => vscode,
            None => {
                return Err(
                    Box::new(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            "Couldn't find [vscode] section in thcon.toml"
                        )
                    )
                );
            }
        };

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };

        let settings = fs::read_to_string(self.settings_json_path())?;
        let settings = replace_color_theme(&settings, theme);
        fs::write(self.settings_json_path(), settings).map_err(|err| {
            Box::new(err) as Box<dyn Error>
        })
    }
}

fn replace_color_theme(settings_json: &str, new_theme: &str) -> String {
    let theme_regex = Regex::new(r#"^(?P<prefix>\s*"workbench.colorTheme"\s*:\s*)"(?P<v>.+)"(?P<suffix>,?\s*//\s*thcon:replace-line)"#);
    match theme_regex {
        Err(_) => settings_json.to_owned(),
        Ok(theme_regex) => {
            let modified_lines: Vec<String> = settings_json.lines().map(|line| {
                theme_regex.replace(line, |caps: &Captures| {
                    format!(r#"{}"{}"{}"#, &caps["prefix"], new_theme, &caps["suffix"])
                }).into_owned()
            }).collect();

            modified_lines.join("\n")
        }
    }
}

#[test]
fn replace_color_trailing_comma() {
    let settings_json = r#"
    {
        "workbench.colorTheme": "Default Dark+", // thcon:replace-line
        "editor.minimap.enabled": false,
    }
    "#;
    let res = replace_color_theme(settings_json, "Default Light+");
    assert_eq!(
        res,
        r#"
    {
        "workbench.colorTheme": "Default Light+", // thcon:replace-line
        "editor.minimap.enabled": false,
    }
    "#
    );
}

#[test]
fn replace_color_no_trailing_comma() {
    let settings_json = r#"
    {
        "editor.minimap.enabled": false,
        "workbench.colorTheme"  :"Default Dark+"   // thcon:replace-line
    }
    "#;
    let res = replace_color_theme(settings_json, "Default Light+");
    assert_eq!(
        res,
        r#"
    {
        "editor.minimap.enabled": false,
        "workbench.colorTheme"  :"Default Light+"   // thcon:replace-line
    }
    "#
    );
}