//! # thcon
//! Switches multiple apps between light and dark mode

#![deny(clippy::all)]

mod config;
mod operation;
mod themeable;

pub mod app;
pub mod dirs;
pub mod sockets;
pub use config::Config;
pub use operation::Operation;
pub use thcon_macro::AppConfig;
pub use thcon_macro::Disableable;
pub use thcon_trait::Disableable;
pub use themeable::{ConfigState, Themeable};
