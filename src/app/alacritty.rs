use std::error::Error;
use std::fs;
use std::env;
use std::io;
use std::path::PathBuf;

use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

#[cfg(windows)]
use dirs;

#[cfg(not(windows))]
use xdg;

use serde::Deserialize;
use regex::{Captures,Regex};

#[derive(Debug, Deserialize)]
pub struct Config {
    light: String,
    dark: String,
    config: Option<String>,
}

pub struct Alacritty;

impl Themeable for Alacritty {
    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.alacritty {
            Some(alacritty) => alacritty,
            None => {
                return Err(
                    Box::new(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            "Couldn't find [alacritty] section in thcon.toml"
                        )
                    )
                );
            }
        };

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
            _ => panic!("Unsupported operation '{}'", operation),
        };

        let alacritty_yaml = match alacritty_config() {
            Some(path) => path,
            None => {
                let couldnt_find = String::from("Couldn't find alacritty.yml");
                let message = match &config.config {
                    Some(_path) => couldnt_find,
                    None => format!("{}; consider adding `config` property to `[alacritty]` section of `thcon.toml`", couldnt_find),
                };

                return Err(
                    Box::new(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            message
                        )
                    )
                );
            }
        };

        let theme_regex = Regex::new(r#"^(?P<prefix>"?colors"?\s*:\s*"?)(?P<v>[^\s]+)(?P<suffix>"?,?\s*#\s*thcon:replace-line)"#)?;
        let settings = fs::read_to_string(&alacritty_yaml)?;
        let modified_lines: Vec<String> = settings.lines().map(|line| {
            theme_regex.replace(line, |caps: &Captures| {
                format!("{}*{}{}", &caps["prefix"], theme, &caps["suffix"])
            }).into_owned()
        }).collect();
        let settings = modified_lines.join("\n");

        fs::write(&alacritty_yaml, settings).map_err(|err| {
            Box::new(err) as Box<dyn Error>
        })
    }

    fn toggle(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
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
    dirs::config_dir().map(|path| path.join("alacritty\\alacritty.yml")).filter(|new| new.exists())
}
