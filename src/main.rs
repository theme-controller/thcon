use std::vec::Vec;

use clap::{Arg, App};
use rayon::prelude::*;

use thcon::Operation;
use thcon::app;
use thcon::Themeable;

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

    let (operation, subcommand) = match matches.subcommand() {
        ("light", Some(subcommand)) => (Operation::Lighten, subcommand),
        ("dark", Some(subcommand)) => (Operation::Darken, subcommand),
        ("invert", Some(subcommand)) => (Operation::Invert, subcommand),
        (other, Some(_)) => panic!("Invalid subcommand name {}", other),
        _ => panic!("Could not find subcommand")
    };

    let maybe_apps = subcommand.values_of("app");
    let app_names: Vec<&str> = maybe_apps.unwrap().collect();
    app_names.par_iter().for_each(|app_name| {
        let app: Box<dyn Themeable> = match app::get(app_name) {
            None => {
                println!("Ignoring unknown app name '{}'", app_name);
                return;
            },
            Some(app) => app,
        };

        app.switch(&operation).unwrap();
    });
}
