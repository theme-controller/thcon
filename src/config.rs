#[cfg(dbus)]
use crate::app::konsole;
#[cfg(dbus)]
use crate::app::gnome_terminal;
use crate::app::vscode;
use crate::app::plasma;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    #[cfg(dbus)]
    pub konsole: Option<konsole::Config>,
    #[cfg(dbus)]
    #[serde(rename = "gnome-terminal")]
    pub gnome_terminal: Option<gnome_terminal::Config>,
    pub vscode: Option<vscode::Config>,
    pub plasma: Option<plasma::Config>
}
