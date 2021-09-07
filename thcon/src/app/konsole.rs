//! Switches between [Konsole](https://konsole.kde.org) profiles
//!
//! ## Usage: Linux & BSD
//! Konsole instances can be discovered and controlled via DBus, but it's a cumbersome process to
//! perform in a one-liner. `thcon` simplifies that - just list the name of the Konsole profile you
//! prefer in light mode and in dark mode in your `thcon.toml`, e.g.:
//!
//! ```toml
//! [plasma]
//! dark = "Some dark profile"
//! light = "A light profile"
//! ```
//!
//! ## Usage: Windows & macOS
//! Konsole is not available on Windows or macOS.
//!
//! ## `thcon.toml` Schema
//! Section: `konsole`
//!
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
//! | `dark` | string | The name of the profile (case-sensitive) to use in dark mode | (none) |
//! | `light` | string | The name of the profile (case-sensitive) to use in light mode | (none) |

use crate::config::Config as ThconConfig;
use crate::operation::Operation;
use crate::AppConfig;
use crate::Disableable;
use crate::{
    themeable::{ConfigError, ConfigState},
    Themeable,
};

use std::time::Duration;

use anyhow::{Context, Result};
use dbus::blocking::Connection;
use log::{debug, trace};
use serde::Deserialize;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    light: String,
    dark: String,
    #[serde(default)]
    disabled: bool,
}

pub struct Konsole {
    dbus: Connection,
}

impl Default for Konsole {
    fn default() -> Self {
        Self {
            dbus: Connection::new_session().unwrap(),
        }
    }
}

impl Konsole {
    fn get_services(&self) -> Result<Vec<String>> {
        let proxy = self
            .dbus
            .with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(2500));
        let (names,): (Vec<String>,) = proxy
            .method_call("org.freedesktop.DBus", "ListNames", ())
            .context("Unable to retrieve konsole windows from DBus")?;

        let konsoles: Vec<String> = names
            .into_iter()
            .filter(|name| name.as_str().starts_with("org.kde.konsole-"))
            .collect();

        trace!(
            "Found {} {}",
            konsoles.len(),
            if konsoles.len() == 1 {
                "service"
            } else {
                "services"
            },
        );

        Ok(konsoles)
    }

    fn get_session_ids(&self, service_id: &str) -> Result<Vec<String>> {
        let proxy = self
            .dbus
            .with_proxy(service_id, "/Sessions", Duration::from_millis(2500));
        let (xml,): (String,) = proxy
            .method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .with_context(|| {
                format!(
                    "Unable to get konsole session ids for DBus service '{}'",
                    service_id
                )
            })?;

        let parser = EventReader::from_str(&xml);
        let mut depth = 0;

        let mut session_ids: Vec<String> = vec![];

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if depth == 1 && name.local_name == "node" {
                        session_ids.extend(attributes.into_iter().filter_map(|attr| {
                            if attr.name.local_name == "name" {
                                Some(attr.value)
                            } else {
                                None
                            }
                        }));
                    }
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { .. }) => depth -= 1,
                Err(e) => {
                    return Err(e.into());
                }
                _ => {}
            }
        }

        trace!(
            "Found {} {} in service {}",
            session_ids.len(),
            if session_ids.len() == 1 {
                "session"
            } else {
                "sessions"
            },
            service_id
        );
        Ok(session_ids)
    }

    fn set_profile_name(
        &self,
        service_id: &str,
        session_id: &str,
        profile_name: &str,
    ) -> Result<()> {
        let proxy = self.dbus.with_proxy(
            service_id,
            format!("/Sessions/{}", session_id),
            Duration::from_millis(2500),
        );
        let _: () = proxy.method_call("org.kde.konsole.Session", "setProfile", (profile_name,))?;

        Ok(())
    }

    fn set_default_profile(&self, service_id: &str, profile_name: &str) -> Result<()> {
        // grab the ID of the first encountered window
        let proxy = self
            .dbus
            .with_proxy(service_id, "/Windows", Duration::from_millis(2500));
        let (xml,): (String,) = proxy
            .method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .with_context(|| {
                format!("Unable to retreive window for DBus service '{}", service_id)
            })?;

        let parser = EventReader::from_str(&xml);
        let mut depth = 0;

        let mut window_id: Option<String> = None;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if depth == 1 && name.local_name == "node" {
                        window_id = attributes.into_iter().find_map(|attr| {
                            if attr.name.local_name == "name" {
                                Some(attr.value)
                            } else {
                                None
                            }
                        });
                    }
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { .. }) => depth -= 1,
                Err(e) => {
                    return Err(e.into());
                }
                _ => {}
            }
        }

        if let Some(window_id) = window_id {
            trace!("Found first window ID {}", window_id);
            let proxy = self.dbus.with_proxy(
                service_id,
                format!("/Windows/{}", window_id),
                Duration::from_millis(2500),
            );
            proxy
                .method_call(
                    "org.kde.konsole.Window",
                    "setDefaultProfile",
                    (profile_name,),
                )
                .context("asdfasdf")?;
        } else {
            trace!("Found no Konsole windows; can't set default profile.");
        }

        Ok(())
    }
}

impl Themeable for Konsole {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_manual_config(config.konsole.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()> {
        let config = match self.config_state(config) {
            ConfigState::NoDefault => {
                return Err(ConfigError::RequiresManualConfig("konsole").into())
            }
            ConfigState::Default => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Enabled => config.konsole.as_ref().unwrap().unwrap_inner_left(),
        };

        let mut total_sessions = 0;
        let services: Vec<(String, Vec<String>)> = self
            .get_services()?
            .into_iter()
            .map(|service| {
                let session_ids = self.get_session_ids(&service).unwrap();
                total_sessions += session_ids.len();
                (service, session_ids)
            })
            .collect();

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };

        if services.len() == 1 {
            debug!(
                "Found {} {}",
                total_sessions,
                if total_sessions == 1 {
                    "session"
                } else {
                    "sessions"
                },
            );
        } else {
            debug!(
                "Found {} {} across {} services",
                total_sessions,
                if total_sessions == 1 {
                    "session"
                } else {
                    "sessions"
                },
                services.len(),
            );
        }

        for (service_id, session_ids) in services.iter() {
            for session_id in session_ids.iter() {
                self.set_profile_name(service_id, session_id, &theme)?;
            }
        }

        if let Some((session, _)) = services.get(0) {
            self.set_default_profile(session, &theme)?;
        }

        Ok(())
    }
}
