use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

use std::vec::Vec;
use std::error::Error;
use std::time::Duration;

use gio::SettingsExt;
use dbus::blocking::Connection;
use dbus::arg::Variant;
use dbus::arg::Dict;
use log::{error, debug};
use serde::Deserialize;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug,Deserialize)]
pub struct Config {
    light: String,
    dark: String
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
    fn get_window_ids(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let proxy = self.dbus.with_proxy("org.gnome.Terminal", "/org/gnome/Terminal/window", Duration::from_millis(2500));
        let (xml,): (String,) = proxy.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())?;

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
                    return Err(Box::new(e));
                },
                _ => {}
            }
        }

        Ok(window_ids)
    }

    fn set_profile(&self, window_id: &str, profile_id: &str) -> Result<(), Box<dyn Error>> {
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
        )?;

        Ok(())
    }
}

impl Themeable for GnomeTerminal {
    fn has_config(&self, config: &ThconConfig) -> bool {
        config.gnome_terminal.is_some()
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.gnome_terminal {
            Some(gnome_terminal) => gnome_terminal,
            None => {
                error!("Couldn't find [gnome_terminal] section in thcon.toml");
                return Ok(());
            }
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
        match gsettings.set_string("default", &theme) {
            Ok(_) => gio::Settings::sync(),
            Err(e) => error!("Unable to set default profile: {}", e),
        }

        Ok(())
    }
}
