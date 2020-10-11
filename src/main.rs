#[macro_use]
extern crate lazy_static;

extern crate clap;
use std::fmt;
use clap::{Arg, App};

#[derive(Debug)]
enum Operation {
    Darken,
    Lighten,
    Invert
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match &self {
            Self::Darken => "darken",
            Self::Lighten => "lighten",
            Self::Invert => "invert",
        };

        write!(f, "{}", as_str)
    }
}


mod app {
    use std::collections::HashMap;
    use std::fmt;
    use crate::Operation;
    struct Config;

    // const GNOME_TERMINAL: Application = Application::Known("gnome-terminal");

    // #[derive(Debug)]
    // pub enum Application {
    //     Known(Themeable),
    //     Unknown(String)
    // }

    // #[derive(Debug)]
    // pub enum KnownApps {
    //     GnomeTerminal
    // }

    // impl fmt::Display for KnownApps {
    //     fn fmt(&self) -> String {
    //         match (&self) {
    //             Self::GnomeTerminal => GNOME_TERMINAL
    //         }
    //     }
    // }
    // pub struct Application(T);

    #[derive(Debug, Clone)]
    pub struct UnsupportedApp{
        app: String
    }

    impl fmt::Display for UnsupportedApp {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            return write!(f, "Unsupported application '{}'", self.app);
        }
    }

    // impl<T: Themeable> std::str::FromStr for Application<T> {
    // impl<T: Themeable> std::str::FromStr for T {
    //     type Err = UnsupportedApp;

    //     fn from_str(s: &str) -> Result<T, Self::Err> {
    //         return match s {
    //             "konsole" => Ok(Konsole::new()),
    //             _ => Err(UnsupportedApp { app: s.to_owned() })
    //         };
    //     }
    // }

    pub trait Themeable: std::fmt::Debug + std::marker::Sync {
        fn switch(&self, operation: &Operation) -> Result<(), ()>;
        fn toggle(&self) -> Result<(), ()>;
        fn parse_config(&self, config: Config) -> Result<(), ()>;
    }

    #[derive(Clone,Debug)]
    struct Konsole {}

    impl Themeable for Konsole {
        fn switch(&self, operation: &Operation) -> Result<(), ()> {
            println!("Switching konsole to {}", operation);
            Result::Ok(())
        }

        fn toggle(&self) -> Result<(), ()> {
            Result::Ok(())
        }

        fn parse_config(&self, config: Config) -> Result<(), ()> {
            Result::Ok(())
        }
    }

    const KONSOLE_INSTANCE: Konsole = Konsole {};
    lazy_static! {
        pub static ref KNOWN_APPS: HashMap<&'static str, &'static dyn Themeable> = [
            ("konsole", &KONSOLE_INSTANCE as &dyn Themeable)
        ].iter().cloned().collect();
    }
}

fn main() {
    let matches = App::new("thcon")
                    .version("1.0")
                    .author("Sean Barag <sean@barag.org>")
                    .subcommand(App::new("light")
                                .display_order(1001)
                                .about("switches to light mode")
                                .arg(Arg::with_name("app")
                                     .help("Application(s) to switch to light mode")
                                     .multiple(true)
                                 )
                    )
                    .subcommand(App::new("dark")
                                .display_order(1000)
                                .about("switches to dark mode")
                                .arg(Arg::with_name("app")
                                     .help("Application(s) to switch to dark mode")
                                     .multiple(true)
                                 )
                    )
                    .subcommand(App::new("invert")
                                .display_order(1002)
                                .about("switches from light mode to dark mode and from dark mode to light mode")
                                .arg(Arg::with_name("app")
                                     .help("Application(s) to switch between dark and light mode")
                                     .multiple(true)
                                 )
                    )
                    .get_matches();

    let subcommand = match matches.subcommand_name() {
        Some("light") => Operation::Lighten,
        Some("dark") => Operation::Darken,
        Some("invert") => Operation::Invert,
        _ => panic!("asdfasdfasdf")
    };

    if let Some(ref matches) = matches.subcommand_matches("dark") {
        let maybe_apps = matches.values_of("app");
        let apps = maybe_apps.unwrap();
        println!("Found matches: {:#?}", apps);
        apps.map(|app_name| {
            match app::KNOWN_APPS.get(app_name) {
                Some(app) => app.to_owned(),
                None => panic!("Found unknown application '{}'", app_name),
            }
        }).for_each(|app| {
            println!("Will darken {:?}", app);
        });
    }
}
