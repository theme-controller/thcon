//! # thcon
//! Switches multiple apps between light and dark mode

mod config;
mod operation;
mod themeable;

pub mod app;
pub use config::Config;
pub use operation::Operation;
pub use themeable::Themeable;
