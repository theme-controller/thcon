#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

pub mod config;
pub mod operation;
pub mod themeable;
pub mod app;

pub use config::Config;
pub use themeable::Themeable;
pub use app::konsole::Konsole;
pub use app::gnome_terminal::GnomeTerminal;

const KONSOLE: Konsole = Konsole {};
const GNOME_TERMINAL: GnomeTerminal = GnomeTerminal {};

lazy_static! {
    pub static ref KNOWN_APPS: HashMap<&'static str, &'static dyn Themeable> = [
        ("konsole", &KONSOLE as &dyn Themeable),
        ("gnome-terminal", &GNOME_TERMINAL as &dyn Themeable)
    ].iter().cloned().collect();
}