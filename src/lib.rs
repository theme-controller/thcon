#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::error::Error;

pub mod config;
pub mod operation;
pub mod themeable;
pub mod app;

pub use config::Config;
pub use themeable::Themeable;
pub use app::konsole::Konsole;
pub use app::gnome_terminal::GnomeTerminal;

pub struct Thcon<'a> {
    pub known_apps: HashMap<&'a str, Box<dyn Themeable>>,
}

pub fn init<'a>() -> Result<Thcon<'a>, Box<dyn Error>> {
    let mut known_apps = HashMap::new();
    known_apps.insert("konsole", Box::new(Konsole::new()?) as Box<dyn Themeable>);

    let instance = Thcon {
        known_apps
    };

    Ok(instance)
}