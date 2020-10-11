use crate::thcon::themeable::Themeable;
use crate::thcon::operation::Operation;
use crate::thcon::config::Config;

#[derive(Clone,Debug)]
pub struct GnomeTerminal {}

impl Themeable for GnomeTerminal {
    fn switch(&self, operation: &Operation) -> Result<(), ()> {
        println!("Switching gnome-terminal to {}", operation);
        Result::Ok(())
    }

    fn toggle(&self) -> Result<(), ()> {
        Result::Ok(())
    }

    fn parse_config(&self, config: Config) -> Result<(), ()> {
        Result::Ok(())
    }
}