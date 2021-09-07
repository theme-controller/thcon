pub mod alacritty;
pub mod atom;
#[cfg(dbus)]
pub mod gnome_shell;
#[cfg(dbus)]
pub mod gnome_terminal;
#[cfg(dbus)]
pub mod gtk;
#[cfg(dbus)]
pub mod konsole;
#[cfg(dbus)]
pub mod plasma;
pub mod sublime_text;
pub mod vim;
pub mod vscode;

#[cfg(mac)]
pub mod iterm2;
#[cfg(mac)]
pub mod macos;
#[cfg(mac)]
pub mod terminal_dot_app;

pub use alacritty::Alacritty;
pub use atom::Atom;
#[cfg(dbus)]
pub use gnome_shell::GnomeShell;
#[cfg(dbus)]
pub use gnome_terminal::GnomeTerminal;
#[cfg(dbus)]
pub use gtk::Gtk;
#[cfg(dbus)]
pub use konsole::Konsole;
#[cfg(dbus)]
pub use plasma::Plasma;
pub use sublime_text::SublimeText;
pub use vim::Neovim;
pub use vim::Vim;
pub use vscode::VSCode;

#[cfg(mac)]
pub use iterm2::Iterm2;
#[cfg(mac)]
pub use macos::MacOS;
#[cfg(mac)]
pub use terminal_dot_app::TerminalDotApp;

use crate::themeable::Themeable;
use std::option::Option;

pub fn get(name: &str) -> Option<Box<dyn Themeable>> {
    match name {
        #[cfg(dbus)]
        "konsole" => Some(Box::new(Konsole::default())),
        #[cfg(dbus)]
        "gnome-shell" => Some(Box::new(GnomeShell {})),
        #[cfg(dbus)]
        "gnome-terminal" => Some(Box::new(GnomeTerminal::default())),
        #[cfg(dbus)]
        "gtk" => Some(Box::new(Gtk {})),
        #[cfg(dbus)]
        "plasma" => Some(Box::new(Plasma {})),
        #[cfg(mac)]
        "macos" => Some(Box::new(MacOS {})),
        #[cfg(mac)]
        "terminal-app" => Some(Box::new(TerminalDotApp {})),
        #[cfg(mac)]
        "iterm2" => Some(Box::new(Iterm2 {})),
        "vscode" => Some(Box::new(VSCode {})),
        "alacritty" => Some(Box::new(Alacritty {})),
        "vim" => Some(Box::new(Vim {})),
        "nvim" => Some(Box::new(Neovim {})),
        "sublime-text" => Some(Box::new(SublimeText {})),
        "atom" => Some(Box::new(Atom {})),
        _ => None,
    }
}

pub fn all_names() -> Vec<&'static str> {
    vec![
        "alacritty",
        "atom",
        #[cfg(dbus)]
        "gnome-shell",
        #[cfg(dbus)]
        "gnome-terminal",
        #[cfg(dbus)]
        "gtk",
        #[cfg(mac)]
        "iterm2",
        #[cfg(dbus)]
        "konsole",
        #[cfg(mac)]
        "macos",
        #[cfg(mac)]
        "terminal-app",
        "nvim",
        #[cfg(dbus)]
        "plasma",
        "sublime-text",
        "vim",
        "vscode",
    ]
}
