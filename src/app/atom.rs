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
//! | `dark` | array of strings | The themes to apply in dark mode, as shown in `config.cson` | (none) |
//! | `light` | array of strings | The themes to apply in dark mode, as shown in `config.cson` | (none) |

use std::error::Error;
use std::io;
use std::io::Write;
use std::os::unix::net::UnixStream;

use serde::{Deserialize, Serialize};

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use crate::sockets;
use crate::themeable::Themeable;

#[derive(Debug, Deserialize)]
pub struct Atom {}

#[derive(Debug, Deserialize)]
pub struct Config {
    dark: Vec<String>,
    light: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct WireConfig {
    #[serde(rename = "core.themes")]
    themes: Vec<String>,
}

impl Themeable for Atom {
    fn has_config(&self, config: &ThconConfig) -> bool {
        config.atom.is_some()
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.atom {
            Some(config) => config,
            None => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Couldn't find [atom] section in thcon.toml"),
                )))
            }
        };

        let themes = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };
        let wire_format = WireConfig {
            themes: themes.to_owned(),
        };
        let payload = serde_json::to_vec(&wire_format).unwrap_or_default();

        let addr = sockets::socket_addr("atom", false);
        if let Ok(mut stream) = UnixStream::connect(&addr) {
            stream.write_all(&payload).unwrap_or(())
        }
        Ok(())
    }
}
