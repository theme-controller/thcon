//! Switches between [Atom](https://atom.io) UI and editor themes in all windows and tabs.
//!
//! ## Usage: All Platforms
//! Install [thcon-atom](https://github.com/theme-controller/thcon-atom) with `apm install thcon`.
//! In `thcon.toml`, define a list of themes to apply in dark mode and light mode.  These can be
//! copy-pasted from the `core.themes` property in your `config.cson`.  You can easily get Atom
//! looking right in dark mode, copy those themes into `thcon.toml`, then repeat for light mode.
//!
//! ```toml
//! [atom]
//! dark = [ "one-dark-ui", "one-dark-syntax" ]
//! light = [ "one-light-ui", "one-light-syntax" ]
//! ```
//!
//! ## `thcon.toml` Schema
//! Section: `atom`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
//! | `dark` | array of strings | The themes to apply in dark mode, as shown in `config.cson` | `["one-dark-ui", "one-dark-syntax"]` |
//! | `light` | array of strings | The themes to apply in dark mode, as shown in `config.cson` | `["one-light-ui", "one-light-syntax"]` |

use std::fs;
use std::io;
use std::io::Write;
use std::os::unix::net::UnixStream;

use anyhow::{Context, Result};
use log::trace;
use serde::{Deserialize, Serialize};

use crate::operation::Operation;
use crate::sockets;
use crate::themeable::Themeable;
use crate::AppConfig;
use crate::Disableable;
use crate::{config::Config as ThconConfig, themeable::ConfigState};

#[derive(Debug, Deserialize)]
pub struct Atom {}

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    dark: Vec<String>,
    light: Vec<String>,
    #[serde(default)]
    disabled: bool,
}

impl Default for _Config {
    fn default() -> Self {
        Self {
            dark: vec!["one-dark-ui", "one-dark-syntax"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            light: vec!["one-light-ui", "one-light-syntax"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            disabled: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WireConfig {
    #[serde(rename = "core.themes")]
    themes: Vec<String>,
}

impl Themeable for Atom {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_default_config(config.atom.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()> {
        let default_config = _Config::default();

        let config = match self.config_state(config) {
            ConfigState::NoDefault => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Default => &default_config,
            ConfigState::Enabled => config.atom.as_ref().unwrap().unwrap_inner_left(),
        };

        let themes = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };
        let wire_format = WireConfig {
            themes: themes.to_owned(),
        };
        let payload = serde_json::to_vec(&wire_format).unwrap_or_default();

        let sock_dir = sockets::socket_addr("atom", true);
        let sock_dir = sock_dir.parent().unwrap();

        let sockets = match fs::read_dir(sock_dir) {
            Ok(sockets) => Ok(Some(sockets)),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    trace!("Found no sockets to write to");
                    Ok(None)
                }
                _ => Err(e),
            },
        }?;

        match sockets {
            None => (),
            Some(sockets) => {
                for sock in sockets {
                    if sock.is_err() {
                        continue;
                    }
                    let sock = sock.unwrap().path();
                    if let Ok(mut stream) = UnixStream::connect(&sock) {
                        trace!("Writing to socket at {}", &sock.display());
                        stream.write_all(&payload).with_context(|| {
                            format!("Unable to write to socket at {}", sock.display())
                        })?;
                    }
                }
            }
        };

        Ok(())
    }
}
