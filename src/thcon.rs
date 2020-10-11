use std::collections::HashMap;
use std::fmt;

pub mod config;
pub mod operation;
pub mod themeable;
pub mod app;

pub use config::Config;
pub use themeable::Themeable;
pub use app::konsole::Konsole;
pub use app::gnome_terminal::GnomeTerminal;

#[derive(Debug, Clone)]
pub struct UnsupportedApp{
    app: String
}

impl fmt::Display for UnsupportedApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Unsupported application '{}'", self.app);
    }
}

const KONSOLE: Konsole = Konsole {};
const GNOME_TERMINAL: GnomeTerminal = GnomeTerminal {};
lazy_static! {
    pub static ref KNOWN_APPS: HashMap<&'static str, &'static dyn Themeable> = [
        ("konsole", &KONSOLE as &dyn Themeable),
        ("gnome-terminal", &GNOME_TERMINAL as &dyn Themeable)
    ].iter().cloned().collect();
}