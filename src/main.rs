extern crate clap;
use clap::{Arg, App};

use thcon::Operation;
use thcon::app;

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
    maybe_apps.unwrap().filter_map(|app_name| {
        app::get(app_name).or_else(|| {
            println!("Ignoring unknown app name '{}'", app_name);
            None
        })
    }).for_each(|app| {
        app.switch(&operation).unwrap();
    });
}
