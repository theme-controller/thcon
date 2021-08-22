use crate::themeable::{ConfigError, ConfigState, Themeable};
use crate::operation::Operation;
use crate::config::Config as ThconConfig;
use crate::Disableable;
use crate::AppConfig;

use std::vec::Vec;
use std::time::Duration;

use anyhow::{Context, Result};
use dbus::blocking::Connection;
use dbus::arg::Variant;
use dbus::arg::Dict;
use gio::SettingsExt;
use log::debug;
use serde::Deserialize;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug,Deserialize, Disableable, AppConfig)]
pub struct _Config {
    light: String,
    dark: String,
    #[serde(default)]
    disabled: bool,
}

pub struct GnomeTerminal {
    dbus: Connection,
}

impl Default for GnomeTerminal {
    fn default() -> Self {
        Self { dbus: Connection::new_session().unwrap(), }
    }
}

impl GnomeTerminal {
    fn get_window_ids(&self) -> Result<Vec<String>> {
        let proxy = self.dbus.with_proxy("org.gnome.Terminal", "/org/gnome/Terminal/window", Duration::from_millis(2500));
        let (xml,): (String,) = proxy.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .context("Unable to retrieve gnome-terminal windows from DBus")?;

        let parser = EventReader::from_str(&xml);
        let mut depth = 0;

        let mut window_ids: Vec<String> = vec!();

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    if depth == 1 && name.local_name == "node" {
                        window_ids.extend( attributes.into_iter()
                            .filter_map(|attr| {
                                if attr.name.local_name == "name" {
                                    Some(attr.value)
                                } else {
                                    None
                                }
                            })
                        );
                    }
                    depth += 1;
                },
                Ok(XmlEvent::EndElement {..}) => depth -= 1,
                Err(e) => {
                    return Err(e.into());
                },
                _ => {}
            }
        }

        Ok(window_ids)
    }

    fn set_profile(&self, window_id: &str, profile_id: &str) -> Result<()> {
        let proxy = self.dbus.with_proxy(
            "org.gnome.Terminal",
            format!("/org/gnome/Terminal/window/{}", window_id),
            Duration::from_millis(2500)
        );

        let asv = Dict::new(vec!() as Vec<(String, Variant<String>)>);
        let _: () = proxy.method_call(
            "org.gtk.Actions",
            "SetState",
            ("profile", Variant(profile_id), asv)
        ).with_context(|| format!("Unable to apply profile '{}' for gnome-terminal window '{}'", profile_id, window_id))?;

        Ok(())
    }
}

impl Themeable for GnomeTerminal {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_manual_config(config.gnome_terminal.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()> {
        let config = match self.config_state(config) {
            ConfigState::NoDefault => return Err(ConfigError::RequiresManualConfig("gnome_terminal").into()),
            ConfigState::Default => unreachable!(),
            ConfigState::Disabled => return Ok(()),
            ConfigState::Enabled => config.gnome_terminal.as_ref().unwrap().unwrap_inner_left(),
        };

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };

        if let Ok(windows) = self.get_window_ids() {
            debug!(
                "Found {} {}",
                windows.len(),
                if windows.len() == 1 { "window" } else { "windows" },
            );
            for window_id in windows.iter() {
                self.set_profile(window_id, &theme)?;
            }
        }

        let gsettings = gio::Settings::new("org.gnome.Terminal.ProfilesList");
        gsettings.set_string("default", &theme)
            .map(|_| gio::Settings::sync())
            .with_context(|| format!("Unable to set default gnome-terminal profile '{}'", theme))
    }
}
