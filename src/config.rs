use crate::app::konsole;
use crate::app::gnome_terminal;
use crate::app::vscode;
use crate::app::plasma;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    pub konsole: Option<konsole::Config>,
    #[serde(rename = "gnome-terminal")]
    pub gnome_terminal: Option<gnome_terminal::Config>,
    pub vscode: Option<vscode::Config>,
    pub plasma: Option<plasma::Config>
}