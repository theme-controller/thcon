use thiserror::Error;
use either::Either;
use anyhow::Result;

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use thcon_trait::{Disableable, Disabled};

pub trait Themeable {
    fn config_state(&self, config: &ThconConfig) -> ConfigState;
    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()>;
}

#[derive(PartialEq, Eq)]
pub enum ConfigState {
    NoDefault,
    Default,
    Disabled,
    Enabled,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0} requires manual configuration")]
    RequiresManualConfig(&'static str),
}

impl ConfigState {
    pub fn with_manual_config<T>(section: Option<Either<&T, &Disabled>>) -> Self where T: Disableable {
        match section.as_ref() {
            None => ConfigState::NoDefault,
            Some(c) => match c {
                Either::Left(t) => match t.disabled() {
                    true => ConfigState::Disabled,
                    false => ConfigState::Enabled,
                },
                Either::Right(d) => match d.disabled() {
                    true => ConfigState::Disabled,
                    false => ConfigState::Default,
                }
            }
        }
    }

    pub fn with_default_config<T>(section: Option<Either<&T, &Disabled>>) -> Self where T: Disableable {
        match section.as_ref() {
            None => ConfigState::Default,
            Some(c) => match c {
                Either::Left(t) => match t.disabled() {
                    true => ConfigState::Disabled,
                    false => ConfigState::Enabled,
                }
                Either::Right(d) => match d.disabled() {
                    true => ConfigState::Disabled,
                    false => ConfigState::Default,
                }
            }
        }
    }
}
