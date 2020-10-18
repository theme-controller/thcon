mod konsole;
mod gnome_terminal;
mod plasma;
mod vscode;

pub use konsole::Konsole;
pub use gnome_terminal::GnomeTerminal;
pub use plasma::Plasma;
pub use vscode::VSCode;

use std::option::Option;
use crate::themeable::Themeable;

pub fn get(name: &str) -> Option<Box<dyn Themeable>> {
    return match name {
        "konsole" => Some(Box::new(Konsole::new())),
        "gnome-terminal" => Some(Box::new(GnomeTerminal::new())),
        "plasma" => Some(Box::new(Plasma::new())),
        "vscode" => Some(Box::new(VSCode {})),
        _ => None,
    };
}