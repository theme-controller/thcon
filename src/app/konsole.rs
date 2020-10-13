use crate::Themeable;
use crate::operation::Operation;
use crate::config::Config;

use std::error::Error;
use std::time::Duration;
use dbus::blocking::Connection;

pub struct Konsole {
    dbus: Connection,
}

impl Konsole {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Konsole {
            dbus: Connection::new_session()?
        })
    }

    fn get_services(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let proxy = self.dbus.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(2500));
        let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

        let konsoles: Vec<String> = names.into_iter().filter(|name| {
            name.as_str().starts_with("org.kde.konsole-")
        }).collect();

        Ok(konsoles)
    }
}

impl Themeable for Konsole {
    fn switch(&self, operation: &Operation) -> Result<(), Box<dyn Error>> {
        println!("Switching konsole to {}", operation);

        let konsoles = self.get_services()?;
        println!("found konsoles: {:?}", konsoles);

        Result::Ok(())
    }

    fn toggle(&self) -> Result<(), Box<dyn Error>> {
        Result::Ok(())
    }

    fn parse_config(&self, config: Config) -> Result<(), ()> {
        Result::Ok(())
    }
}