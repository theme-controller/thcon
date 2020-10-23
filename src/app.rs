#[cfg(dbus)]
pub mod konsole;
#[cfg(dbus)]
pub mod gnome_terminal;
pub mod plasma;
pub mod vscode;

#[cfg(dbus)]
pub use konsole::Konsole;
#[cfg(dbus)]
pub use gnome_terminal::GnomeTerminal;
pub use plasma::Plasma;
pub use vscode::VSCode;

use std::option::Option;
use crate::themeable::Themeable;

pub fn get(name: &str) -> Option<Box<dyn Themeable>> {
    return match name {
        #[cfg(dbus)]
        "konsole" => Some(Box::new(Konsole::new())),
        #[cfg(dbus)]
        "gnome-terminal" => Some(Box::new(GnomeTerminal::new())),
        "plasma" => Some(Box::new(Plasma {})),
        "vscode" => Some(Box::new(VSCode {})),
        _ => None,
    };
}
