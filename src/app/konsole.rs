use crate::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

use xml::reader::{EventReader, XmlEvent};

use std::error::Error;
use std::io;
use std::time::Duration;
use dbus::blocking::Connection;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    light: String,
    dark: String,
}

pub struct Konsole {
    dbus: Connection,
}

impl Konsole {
    pub fn new() -> Self {
        Konsole {
            dbus: Connection::new_session().unwrap(),
        }
    }

    fn get_services(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let proxy = self.dbus.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(2500));
        let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

        let konsoles: Vec<String> = names.into_iter().filter(|name| {
            name.as_str().starts_with("org.kde.konsole-")
        }).collect();

        Ok(konsoles)
    }

    fn get_session_ids(&self, service_id: &String) -> Result<Vec<String>, Box<dyn Error>> {
        let proxy = self.dbus.with_proxy(service_id, "/Sessions", Duration::from_millis(2500));
        let (xml,): (String,) = proxy.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())?;

        let parser = EventReader::from_str(&xml);
        let mut depth = 0;

        let mut session_ids: Vec<String> = vec!();

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    if depth == 1 && name.local_name == "node" {
                        session_ids.extend( attributes.into_iter()
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

        Ok(session_ids)
    }

    fn set_profile_name(&self, service_id: &String, session_id: &String, profile_name: &String) -> Result<(), Box<dyn Error>> {
        let proxy = self.dbus.with_proxy(service_id, format!("/Sessions/{}", session_id), Duration::from_millis(2500));
        let _: () = proxy.method_call("org.kde.konsole.Session", "setProfile", (profile_name,))?;

        Ok(())
    }

    fn set_default_profile(&self, profile_name: &String) -> Result<(), Box<dyn Error>> {

        Ok(())
    }
}

impl Themeable for Konsole {
    fn has_config(&self, config: &ThconConfig) -> bool {
        config.konsole.is_some()
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.konsole {
            Some(konsole) => konsole,
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

        let sessions: Vec<(String, Vec<String>)> = self.get_services()?.into_iter()
            .map(|session| {
                let session_ids = self.get_session_ids(&session).unwrap();
                (session, session_ids)
            })
            .collect();

        let theme = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light,
        };
        for (service_id, session_ids) in sessions.iter() {
            for session_id in session_ids.iter() {
                self.set_profile_name(service_id, session_id, &theme)?;
            }
        }

        Ok(())
    }
}
