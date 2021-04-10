//! Switches between [iTerm2](https://iterm2.com) profiles in all windows, tabs, and sessions.
//!
//! ## Usage: macOS
//! Install [thcon-iterm2](https://github.com/sjbarag/thcon-iterm2) by downloading its source and
//! running `make install`.  In your `thcon.toml`, list the name of the profiles to use in dark
//! mode and light mode:
//!
//! ```toml
//! [iterm2]
//! dark = "dark and brooding"
//! light = "light and jovial"
//! ```
//!
//! ## Usage: Windows, Linux & BSD
//! iTerm2 is only available on macOS, so this module is only usable on macOS as a result.
//!
//! ## `thcon.toml` Schema
//! Section: `iterm2`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | `dark` | string | The name of the profile to use in dark mode | (none) |
//! | `light` | string | The name of the profile to use in light mode | (none) |

use std::error::Error;
use std::io;
use std::io::Write;
use std::os::unix::net::UnixStream;

use log::{error, trace};
use serde::{Serialize, Deserialize};

use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;
use crate::sockets;

#[derive(Debug, Deserialize)]
pub struct Config {
    dark: String,
    light: String,
}

#[derive(Debug, Serialize)]
pub struct WireConfig {
    profile: String
}

pub struct Iterm2;
impl Themeable for Iterm2 {
    fn has_config(&self, config: &ThconConfig) -> bool {
        config.iterm2.is_some()
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.iterm2 {
            Some(iterm2) => iterm2,
            None => {
                error!("Couldn't find [iterm2] section in thcon.toml");
                Ok(())
            }
        };

        let profile_name = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light
        };
        let wire_format = WireConfig{ profile: profile_name.to_string() };
        let payload = serde_json::to_vec(&wire_format).unwrap_or_default();

        let addr = sockets::socket_addr("iterm2", false);
        if let Ok(mut stream) = UnixStream::connect(&addr) {
            trace!("Writing to socket at {}", &addr.display());
            stream.write_all(&payload).unwrap_or(())
        }
        Ok(())
    }
}
