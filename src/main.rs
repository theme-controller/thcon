use std::vec::Vec;
use std::path::PathBuf;

use clap::{Arg, App};
use rayon::prelude::*;
use indicatif::{ProgressBar,ProgressStyle};

use thcon::Operation;
use thcon::app;
use thcon::Config;
use thcon::Themeable;

use dirs;
use std::fs;
use toml;

fn main() -> std::io::Result<()> {
    let matches = App::new("thcon")
                    .version("0.3.0")
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
                    .get_matches();

    let config_path: PathBuf = [
            dirs::config_dir().unwrap().to_str().unwrap(),
            "thcon",
            "thcon.toml"
        ].iter().collect();
    println!("reading config from '{:?}'", config_path);
    let config = fs::read_to_string(config_path).unwrap_or(String::from(""));
    let config: Config = toml::from_str(config.as_str())?;

    let (operation, subcommand) = match matches.subcommand() {
        ("light", Some(subcommand)) => (Operation::Lighten, subcommand),
        ("dark", Some(subcommand)) => (Operation::Darken, subcommand),
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

        if app.has_config(&config) {
            app.switch(&config, &operation).unwrap();
        }

        pb.inc(1);
    });

    pb.finish_and_clear();

    Ok(())
}
