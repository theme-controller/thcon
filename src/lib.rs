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
pub use themeable::Themeable;
