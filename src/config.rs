#[cfg(dbus)]
use crate::app::konsole;
#[cfg(dbus)]
use crate::app::gnome_terminal;
#[cfg(dbus)]
use crate::app::plasma;
use crate::app::vscode;
use crate::app::alacritty;
use crate::app::vim;
#[cfg(mac)]
use crate::app::iterm2;
use crate::app::sublime_text;
use crate::app::atom;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    #[cfg(dbus)]
    pub plasma: Option<plasma::Config>,
    #[cfg(dbus)]
    pub konsole: Option<konsole::Config>,
    #[cfg(dbus)]
    #[serde(rename = "gnome-terminal")]
    pub gnome_terminal: Option<gnome_terminal::Config>,
    pub vscode: Option<vscode::Config>,
    pub alacritty: Option<alacritty::Config>,
    pub vim: Option<vim::Config>,
    pub nvim: Option<vim::Config>,
    #[cfg(mac)]
    pub iterm2: Option<iterm2::Config>,
    #[serde(rename = "sublime-text")]
    pub sublime_text: Option<sublime_text::Config>,
    pub atom: Option<atom::Config>,
}
