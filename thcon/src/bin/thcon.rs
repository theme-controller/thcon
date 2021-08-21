use std::vec::Vec;
use std::path::PathBuf;
use std::env;
use std::io;
use std::process;

use anyhow::{anyhow, Result};
use clap::{Arg, App, AppSettings, SubCommand, crate_version};
use rayon::prelude::*;
use log::{error, info, trace, LevelFilter};
use env_logger::TimestampPrecision;

use thcon::Operation;
use thcon::app;
use thcon::Config;
use thcon::ConfigState;

use std::fs;

fn main() -> Result<()> {
    let matches = App::new("thcon")
                    .version(crate_version!())
                    .author("Sean Barag <sean@barag.org>")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .arg(Arg::with_name("verbose")
                            .short("v")
                            .multiple(true)
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

    env_logger::builder()
        .filter_level(match matches.occurrences_of("verbose") {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .format_timestamp(match matches.occurrences_of("verbose") {
            0 | 1 => None,
            _ => Some(TimestampPrecision::Millis)
        })
        .format_target(true)
        .init();

    let (operation, subcommand) = match matches.subcommand() {
        ("light", Some(subcommand)) => (Operation::Lighten, subcommand),
        ("dark", Some(subcommand)) => (Operation::Darken, subcommand),
        _ => unreachable!()
    };

    let has_explicit_apps = subcommand.values_of("app").unwrap_or_default().len() > 0;
    let app_names: Vec<&str> = match subcommand.values_of("app") {
        Some(apps) => apps.collect(),
        None => app::all_names()
    };

    let config_path: PathBuf = [
            thcon::dirs::config().unwrap().to_str().unwrap(),
            "thcon",
            "thcon.toml"
        ].iter().collect();

    trace!("reading config from {}", config_path.display());

    let config = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => {
                info!("Could not find config file at {}; using defaults", config_path.display());
                "".to_string()
            },
            io::ErrorKind::PermissionDenied => {
                error!("Could not read config file from {}", config_path.display());
                process::exit(exitcode::CONFIG);
            },
            _ => {
                error!("Unexpected error while reading config from {}: {}", e, config_path.display());
                process::exit(exitcode::CONFIG);
            }
        }
    });

    let config: Config = match toml::from_str(config.as_str()) {
        Ok(config) => config,
        Err(e) => {
            error!("Encountered invalid TOML in config from {} at {}", config_path.display(), e);
            process::exit(exitcode::CONFIG);
        }
    };

    let has_errors = app_names.par_iter().map(|&name| {
        let app = match app::get(name) {
            None => {
                return Ok(());
            },
            Some(app) => app,
        };

        match app.config_state(&config) {
            ConfigState::NoDefault => {
                if has_explicit_apps {
                    error!(target: name, "skipping (needs manual configuration)");
                    Err(anyhow!("skipping {} (needs manual configuration)", name))
                } else {
                    info!(target: name, "skipping (needs manual configuration)");
                    Ok(())
                }
            },
            ConfigState::Disabled => {
                info!(target: name, "skipping (disabled)");
                Ok(())
            },
            ConfigState::Default => {
                info!(target: name, "{}ing (default configuration)", operation);
                let res = app.switch(&config, &operation);
                if let Err(ref e) = res {
                    error!(target: name, "{:#}", e);
                }
                res
            },
            ConfigState::Enabled => {
                info!(target: name, "{}ing", operation);
                let res = app.switch(&config, &operation);
                if let Err(ref e) = res {
                    error!(target: name, "{:#}", e);
                }
                res
            }
        }
    }).any(|r| r.is_err());

    if has_errors {
        Err(anyhow!("Unable to switch themes"))
    } else {
        Ok(())
    }
}
