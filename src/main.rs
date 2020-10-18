use std::vec::Vec;

use clap::{Arg, App};
use rayon::prelude::*;
use indicatif::{ProgressBar,ProgressStyle};

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

    let app_names: Vec<&str> = subcommand.values_of("app").unwrap().collect();

    let pb = ProgressBar::new(app_names.len() as u64)
        .with_style(ProgressStyle::default_bar());

    app_names.par_iter().for_each(|name| {
        let app: Box<dyn Themeable> = match app::get(name) {
            None => {
                return;
            },
            Some(app) => app,
        };

        app.switch(&operation).unwrap();
        pb.inc(1);
    });

    pb.finish_and_clear();
}
