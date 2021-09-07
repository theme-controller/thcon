#![deny(clippy::all)]

use serde::Deserialize;

/// An app that can be disabled via configuration.
pub trait Disableable {
    /// Returns `true` if the configured app is disabled.
    fn disabled(&self) -> bool;
}

#[derive(Deserialize, Debug)]
pub struct Disabled {
    #[serde(default)]
    disabled: bool,
}

impl Disableable for Disabled {
    fn disabled(&self) -> bool {
        self.disabled
    }
}

/// Unused marker trait to enable #[derive(AppConfig)].
pub trait AppConfig {}
