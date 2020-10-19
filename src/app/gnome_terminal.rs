use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

use xml::reader::{EventReader, XmlEvent};

use std::vec::Vec;
use std::io;
use std::error::Error;
use std::time::Duration;
use dbus::blocking::Connection;
use dbus::arg::Variant;
use dbus::arg::Dict;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    light: String,
    dark: String
}

pub struct GnomeTerminal {
    dbus: Connection,
}

impl GnomeTerminal {
    pub fn new() -> Self {
        GnomeTerminal {
            dbus: Connection::new_session().unwrap(),
        }
    }

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
                                    return Some(attr.value);
                                } else {
                                    return None;
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

    fn set_profile(&self, window_id: &String, profile_id: &String) -> Result<(), Box<dyn Error>> {
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
    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.gnome_terminal {
            Some(gnome_terminal) => gnome_terminal,
            None => {
                return Err(
                    Box::new(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            "Couldn't find [plasma] section in thcon.toml"
                        )
                    )
                );
            }
        };

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
            _ => panic!("Unsupported operation {}", operation),
        };

        for window_id in self.get_window_ids()?.iter() {
            self.set_profile(window_id, &theme)?;
        }

        Result::Ok(())
    }

    fn toggle(&self) -> Result<(), Box<dyn Error>> {
        Result::Ok(())
    }
}