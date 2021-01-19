#[cfg(dbus)]
use crate::app::konsole;
#[cfg(dbus)]
use crate::app::gnome_terminal;
#[cfg(dbus)]
use crate::app::plasma;
use crate::app::vscode;
use crate::app::alacritty;
use crate::app::vim;
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
}
