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
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
//! | `dark` | string | The name of the profile to use in dark mode | (none) |
//! | `light` | string | The name of the profile to use in light mode | (none) |

use std::error::Error;
use std::io::Write;
use std::os::unix::net::UnixStream;

use log::trace;
use serde::{Deserialize, Serialize};

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use crate::sockets;
use crate::themeable::{ConfigError, ConfigState, Themeable};
use crate::AppConfig;
use crate::Disableable;

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    dark: String,
    light: String,
    #[serde(default)]
    disabled: bool,
}

#[derive(Debug, Serialize)]
pub struct WireConfig {
    profile: String,
}

pub struct Iterm2;
impl Themeable for Iterm2 {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_manual_config(config.iterm2.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match self.config_state(config) {
            ConfigState::NoDefault => {
                return Err(Box::from(ConfigError::RequiresManualConfig("iterm2")))
            }
            ConfigState::Default => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Enabled => config.iterm2.as_ref().unwrap().unwrap_inner_left(),
        };

        let profile_name = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };
        let wire_format = WireConfig {
            profile: profile_name.to_string(),
        };
        let payload = serde_json::to_vec(&wire_format).unwrap_or_default();

        let addr = sockets::socket_addr("iterm2", false);
        if let Ok(mut stream) = UnixStream::connect(&addr) {
            trace!("Writing to socket at {}", &addr.display());
            stream.write_all(&payload).unwrap_or(())
        }
        Ok(())
    }
}
