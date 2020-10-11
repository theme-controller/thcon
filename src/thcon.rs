use std::collections::HashMap;
use std::fmt;

pub mod config;
pub mod operation;
pub mod themeable;
pub mod app;

pub use config::Config;
pub use themeable::Themeable;
pub use app::konsole::Konsole;

#[derive(Debug, Clone)]
pub struct UnsupportedApp{
    app: String
}

impl fmt::Display for UnsupportedApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Unsupported application '{}'", self.app);
    }
}

const KONSOLE_INSTANCE: Konsole = Konsole {};
lazy_static! {
    pub static ref KNOWN_APPS: HashMap<&'static str, &'static dyn Themeable> = [
        ("konsole", &KONSOLE_INSTANCE as &dyn Themeable)
    ].iter().cloned().collect();
}