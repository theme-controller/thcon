use crate::Themeable;
use crate::operation::Operation;
use crate::config::Config;

use std::error::Error;
use std::process::Command;

pub struct Plasma;

impl Plasma {
    pub fn new() -> Self {
        Plasma {}
    }
}

impl Themeable for Plasma {
    fn switch(&self, operation: &Operation) -> Result<(), Box<dyn Error>> {
        println!("Switching plasma to {}", operation);

        let theme = match operation {
            Operation::Lighten => "org.fedoraproject.fedora.desktop",
            Operation::Darken => "org.kde.breezedark.desktop",
            _ => panic!("Unsupported operation {}", operation),
        };

        Command::new("lookandfeeltool")
            .arg("--apply")
            .arg(theme)
            .status()
            .expect("Failed to execute `lookandfeeltool`");

        Ok(())
    }

    fn toggle(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn parse_config(&self, config: Config) -> Result<(), ()> {
        Ok(())
    }
}