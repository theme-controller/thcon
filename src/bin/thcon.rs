use std::vec::Vec;
use std::path::PathBuf;
use std::env;
use std::io;
use std::process;

use clap::{Arg, App, AppSettings, SubCommand, crate_version};
use rayon::prelude::*;

use thcon::Operation;
use thcon::app;
use thcon::Config;

use std::fs;

fn main() -> std::io::Result<()> {
    let matches = App::new("thcon")
                    .version(crate_version!())
                    .author("Sean Barag <sean@barag.org>")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .help("Enables verbose output")
                    )
                    .subcommand(SubCommand::with_name("light")
                                .display_order(101)
                                .about("switches to light mode")
                                .arg(Arg::with_name("app")
                                     .help("Application(s) to switch to light mode")
                                     .possible_values(&app::all_names())
                                     .hide_possible_values(true)
                                     .multiple(true)
                                 )
                    )
                    .subcommand(SubCommand::with_name("dark")
                                .display_order(100)
                                .about("switches to dark mode")
                                .arg(Arg::with_name("app")
                                     .help("Application(s) to switch to dark mode")
                                     .possible_values(&app::all_names())
                                     .hide_possible_values(true)
                                     .multiple(true)
                                 )
                    )
                    .get_matches();

    let is_verbose = matches.is_present("verbose");

    let (operation, subcommand) = match matches.subcommand() {
        ("light", Some(subcommand)) => (Operation::Lighten, subcommand),
        ("dark", Some(subcommand)) => (Operation::Darken, subcommand),
        _ => unreachable!()
    };

    let app_names: Vec<&str> = match subcommand.values_of("app") {
        Some(apps) => apps.collect(),
        None => app::all_names()
    };

    let config_path: PathBuf = [
            thcon::dirs::config().unwrap().to_str().unwrap(),
            "thcon",
            "thcon.toml"
        ].iter().collect();

    if is_verbose {
        eprintln!("reading config from {}", config_path.display());
    }

    let config = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => eprintln!("Could not find config file at {}", config_path.display()),
            io::ErrorKind::PermissionDenied => eprintln!("Could not read config file from {}", config_path.display()),
            _ => eprintln!("Unexpected error while reading config from {}: {}", e, config_path.display()),
        };
        process::exit(exitcode::CONFIG);
    });

    let config: Config = match toml::from_str(config.as_str()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Encountered invalid TOML in config from {} at {}", config_path.display(), e);
            process::exit(exitcode::CONFIG);
        }
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
