use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config;

use std::error::Error;

#[derive(Clone,Debug)]
pub struct GnomeTerminal {}

impl Themeable for GnomeTerminal {
    fn switch(&self, operation: &Operation) -> Result<(), Box<dyn Error>> {
        println!("Switching gnome-terminal to {}", operation);
        Result::Ok(())
    }

    fn toggle(&self) -> Result<(), Box<dyn Error>> {
        Result::Ok(())
    }

    fn parse_config(&self, config: Config) -> Result<(), ()> {
        Result::Ok(())
    }
}