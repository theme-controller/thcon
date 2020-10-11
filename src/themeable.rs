use crate::config::Config;
use crate::operation::Operation;


pub trait Themeable: std::fmt::Debug + std::marker::Sync {
    fn switch(&self, operation: &Operation) -> Result<(), ()>;
    fn toggle(&self) -> Result<(), ()>;
    fn parse_config(&self, config: Config) -> Result<(), ()>;
}