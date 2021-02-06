#[cfg(dbus)]
pub mod konsole;
#[cfg(dbus)]
pub mod gnome_terminal;
#[cfg(dbus)]
pub mod plasma;
pub mod vscode;
pub mod vim;
pub mod alacritty;

#[cfg(mac)]
pub mod macos;

#[cfg(mac)]
pub mod iterm2;

#[cfg(dbus)]
pub use konsole::Konsole;
#[cfg(dbus)]
pub use gnome_terminal::GnomeTerminal;
#[cfg(dbus)]
pub use plasma::Plasma;
pub use vscode::VSCode;
pub use alacritty::Alacritty;
pub use vim::Vim;
pub use vim::Neovim;

#[cfg(mac)]
pub use macos::MacOS;
#[cfg(mac)]
pub use iterm2::Iterm2;

use std::option::Option;
use crate::themeable::Themeable;

pub fn get(name: &str) -> Option<Box<dyn Themeable>> {
    match name {
        #[cfg(dbus)]
        "konsole" => Some(Box::new(Konsole::default())),
        #[cfg(dbus)]
        "gnome-terminal" => Some(Box::new(GnomeTerminal::default())),
        #[cfg(dbus)]
        "plasma" => Some(Box::new(Plasma {})),
        #[cfg(mac)]
        "macos" => Some(Box::new(MacOS {})),
        #[cfg(mac)]
        "iterm2" => Some(Box::new(Iterm2 {})),
        "vscode" => Some(Box::new(VSCode {})),
        "alacritty" => Some(Box::new(Alacritty {})),
        "vim" => Some(Box::new(Vim {})),
        "nvim" => Some(Box::new(Neovim {})),
        _ => None,
    }
}

pub fn all_names() -> Vec<&'static str> {
    vec!(
        "alacritty",
        "nvim",
        "vim",
        "vscode",
        #[cfg(dbus)]
        "gnome-terminal",
        #[cfg(dbus)]
        "konsole",
        #[cfg(dbus)]
        "plasma",
        #[cfg(mac)]
        "macos",
        #[cfg(mac)]
        "iterm2",
    )
}
