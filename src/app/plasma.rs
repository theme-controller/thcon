use crate::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

use std::error::Error;
use std::io;
use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    light: String,
    dark: String,
}

pub struct Plasma;

impl Themeable for Plasma {
    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.plasma {
            Some(plasma) => plasma,
            None => {
                return Err(
                    Box::new(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            "Couldn't find [plasma] section in thcon.toml"
                        )
                    )
                );
            }
        };

        let theme = match operation {
            Operation::Lighten => &config.light,
            Operation::Darken => &config.dark,
        };

        Command::new("lookandfeeltool")
            .arg("--apply")
            .arg(theme)
            .status()
            .expect("Failed to execute `lookandfeeltool`");

        Ok(())
    }
}
