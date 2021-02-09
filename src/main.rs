use std::vec::Vec;
use std::path::PathBuf;
use std::env;

use clap::{Arg, App, crate_version};
use rayon::prelude::*;

use thcon::Operation;
use thcon::app;
use thcon::Config;

use std::fs;

fn main() -> std::io::Result<()> {
    let matches = App::new("thcon")
                    .version(crate_version!())
                    .author("Sean Barag <sean@barag.org>")
                    .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .help("Enables verbose output")
                    )
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
                    .get_matches();

    let is_verbose = matches.is_present("verbose");

    let config_path: PathBuf = [
            thcon::dirs::config().unwrap().to_str().unwrap(),
            "thcon",
            "thcon.toml"
        ].iter().collect();

    if is_verbose {
        eprintln!("reading config from '{:?}'", config_path);
    }

    let config = fs::read_to_string(config_path).unwrap_or_default();
    let config: Config = toml::from_str(config.as_str())?;

    let (operation, subcommand) = match matches.subcommand() {
        ("light", Some(subcommand)) => (Operation::Lighten, subcommand),
        ("dark", Some(subcommand)) => (Operation::Darken, subcommand),
        (other, Some(_)) => panic!("Invalid subcommand name {}", other),
        _ => panic!("Could not find subcommand")
    };

    let app_names: Vec<&str> = match subcommand.values_of("app") {
        Some(apps) => apps.collect(),
        None => app::all_names()
    };

    app_names.par_iter().for_each(|name| {
        let app = match app::get(name) {
            None => {
                return;
            },
            Some(app) => app,
        };

        if app.has_config(&config) {
            if is_verbose {
                eprintln!("{}ing {}", operation, name);
            }
            app.switch(&config, &operation).unwrap();
        } else if is_verbose {
            eprintln!("skipping {} (not configured)", name);
        }
    });

    Ok(())
}
