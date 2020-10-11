extern crate clap;
use clap::{Arg, App};

use thcon::operation::Operation;

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
        maybe_apps.unwrap().map(|app_name| {
            match thcon::KNOWN_APPS.get(app_name) {
                Some(app) => app.to_owned(),
                None => panic!("Found unknown application '{}'", app_name),
            }
        }).for_each(|app| {
            app.switch(&subcommand).unwrap();
        });
    }
}
