use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use dirs;
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
            _ => panic!("Unsupported operation '{}'", operation),
        };
        let theme_regex = Regex::new(r#"^(?P<prefix>\s*"workbench.colorTheme"\s*:\s*)"(?P<v>.+)",?(?P<suffix>\s*//\s*thcon:replace-line)"#)?;

        let settings = fs::read_to_string(self.settings_json_path())?;
        let modified_lines: Vec<String> = settings.lines().map(|line| {
            theme_regex.replace(line, |caps: &Captures| {
                format!(r#"{}"{}"{}"#, &caps["prefix"], theme, &caps["suffix"])
            }).into_owned()
        }).collect();
        let settings = modified_lines.join("\n");

        fs::write(self.settings_json_path(), settings).map_err(|err| {
            Box::new(err) as Box<dyn Error>
        })
    }

    fn toggle(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}