#[cfg(dbus)]
use crate::app::konsole;
#[cfg(dbus)]
use crate::app::gnome_shell;
#[cfg(dbus)]
use crate::app::gnome_terminal;
#[cfg(dbus)]
use crate::app::gtk;
#[cfg(dbus)]
use crate::app::plasma;
use crate::app::vscode;
use crate::app::alacritty;
#[cfg(not(windows))]
use crate::app::vim;
#[cfg(mac)]
use crate::app::iterm2;
#[cfg(mac)]
use crate::app::macos;
#[cfg(mac)]
use crate::app::terminal_dot_app;
use crate::app::sublime_text;
#[cfg(not(windows))]
use crate::app::atom;
#[cfg(windows)]
use crate::app::windows;

use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    #[cfg(dbus)]
    pub plasma: Option<plasma::Config>,
    #[cfg(dbus)]
    pub konsole: Option<konsole::Config>,
    #[cfg(dbus)]
    #[serde(rename = "gnome-shell")]
    pub gnome_shell: Option<gnome_shell::Config>,
    #[cfg(dbus)]
    #[serde(rename = "gnome-terminal")]
    pub gnome_terminal: Option<gnome_terminal::Config>,
    #[cfg(dbus)]
    pub gtk: Option<gtk::Config>,
    pub vscode: Option<vscode::Config>,
    pub alacritty: Option<alacritty::Config>,
    #[cfg(not(windows))]
    pub vim: Option<vim::Config>,
    #[cfg(not(windows))]
    pub nvim: Option<vim::Config>,
    #[cfg(mac)]
    pub iterm2: Option<iterm2::Config>,
    #[cfg(mac)]
    pub macos: Option<macos::Config>,
    #[cfg(mac)]
    #[serde(rename = "terminal-app")]
    pub terminal_dot_app: Option<terminal_dot_app::Config>,
    #[serde(rename = "sublime-text")]
    pub sublime_text: Option<sublime_text::Config>,
    #[cfg(not(windows))]
    pub atom: Option<atom::Config>,
    #[cfg(windows)]
    pub windows: Option<windows::Config>,
}
