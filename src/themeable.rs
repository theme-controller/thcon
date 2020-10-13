use crate::config::Config;
use crate::operation::Operation;

use std::error::Error;

pub trait Themeable {
    fn switch(&self, operation: &Operation) -> Result<(), Box<dyn Error>>;
    fn toggle(&self) -> Result<(), Box<dyn Error>>;
    fn parse_config(&self, config: Config) -> Result<(), ()>;
}