use crate::thcon::Themeable;
use crate::thcon::operation::Operation;
use crate::thcon::config::Config;

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
