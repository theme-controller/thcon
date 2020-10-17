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
pub use app::plasma::Plasma;

pub struct Thcon<'a> {
    pub known_apps: HashMap<&'a str, Box<dyn Themeable>>,
}

pub fn init<'a>() -> Result<Thcon<'a>, Box<dyn Error>> {
    let mut known_apps = HashMap::new();
    known_apps.insert("konsole", Box::new(Konsole::new()?) as Box<dyn Themeable>);
    known_apps.insert("gnome-terminal", Box::new(GnomeTerminal::new()?) as Box<dyn Themeable>);
    known_apps.insert("plasma", Box::new(Plasma::new()) as Box<dyn Themeable>);

    let instance = Thcon {
        known_apps
    };

    Ok(instance)
}