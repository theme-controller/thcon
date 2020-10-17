use crate::Themeable;
use crate::operation::Operation;
use crate::config::Config;

use xml::reader::{EventReader, XmlEvent};

use std::error::Error;
use std::time::Duration;
use dbus::blocking::Connection;

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
    fn switch(&self, operation: &Operation) -> Result<(), Box<dyn Error>> {
        println!("Switching konsole to {}", operation);

        let service_ids = self.get_services()?;
        println!("found konsoles: {:?}", service_ids);

        let sessions: Vec<(String, Vec<String>)> = service_ids.into_iter()
            .map(|session| {
                let session_ids = self.get_session_ids(&session).unwrap();
                (session, session_ids)
            })
            .collect();
        println!("found sessions: {:?}", sessions);

        let theme = match operation {
            Operation::Darken => String::from("Profile 1"),
            Operation::Lighten => String::from("zipper"),
            _ => panic!("Unsupported operation {}", operation),
        };
        for (service_id, session_ids) in sessions.iter() {
            for session_id in session_ids.iter() {
                self.set_profile_name(service_id, session_id, &theme)?;
            }
        }

        Ok(())
    }

    fn toggle(&self) -> Result<(), Box<dyn Error>> {
        Result::Ok(())
    }

    fn parse_config(&self, config: Config) -> Result<(), ()> {
        Result::Ok(())
    }
}