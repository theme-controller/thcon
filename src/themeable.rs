use crate::config::Config;
use crate::operation::Operation;

use std::error::Error;

pub trait Themeable {
    fn has_config(&self, config: &Config) -> bool;
    fn switch(&self, config: &Config, operation: &Operation) -> Result<(), Box<dyn Error>>;
}
