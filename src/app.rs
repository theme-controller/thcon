#[cfg(dbus)]
pub mod konsole;
#[cfg(dbus)]
pub mod gnome_terminal;
pub mod plasma;
pub mod vscode;
pub mod vim;
pub mod alacritty;

#[cfg(macos)]
pub mod macos;

#[cfg(dbus)]
pub use konsole::Konsole;
#[cfg(dbus)]
pub use gnome_terminal::GnomeTerminal;
pub use plasma::Plasma;
pub use vscode::VSCode;
pub use alacritty::Alacritty;
pub use vim::Vim;
pub use vim::Neovim;

#[cfg(macos)]
pub use macos::MacOS;

use std::option::Option;
use crate::themeable::Themeable;

pub fn get(name: &str) -> Option<Box<dyn Themeable>> {
    return match name {
        #[cfg(dbus)]
        "konsole" => Some(Box::new(Konsole::new())),
        #[cfg(dbus)]
        "gnome-terminal" => Some(Box::new(GnomeTerminal::new())),
        #[cfg(macos)]
        "macos" => Some(Box::new(MacOS {})),
        "plasma" => Some(Box::new(Plasma {})),
        "vscode" => Some(Box::new(VSCode {})),
        "alacritty" => Some(Box::new(Alacritty {})),
        "vim" => Some(Box::new(Vim {})),
        "nvim" => Some(Box::new(Neovim {})),
        _ => None,
    };
}
