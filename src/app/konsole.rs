use crate::Themeable;
use crate::operation::Operation;
use crate::config::Config;

#[derive(Clone,Debug)]
pub struct Konsole {}

impl Themeable for Konsole {
    fn switch(&self, operation: &Operation) -> Result<(), ()> {
        println!("Switching konsole to {}", operation);
        Result::Ok(())
    }

    fn toggle(&self) -> Result<(), ()> {
        Result::Ok(())
    }

    fn parse_config(&self, config: Config) -> Result<(), ()> {
        Result::Ok(())
    }
}
