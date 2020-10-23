use crate::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

use std::error::Error;
use std::process::Command;

pub struct MacOS;

impl Themeable for MacOS {
    fn switch(&self, _config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let dark_mode = match operation {
            Operation::Lighten => false,
            Operation::Darken => true,
            _ => panic!("Unsupported operation {}", operation),
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

    fn toggle(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
