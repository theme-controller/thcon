use std::error::Error;
use std::io;
use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::{Value as JsonValue, Map};

use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    dark: ConfigSection,
    light: ConfigSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSection {
    colorscheme: Option<String>,
    r#let: Option<Map<String, JsonValue>>,
    set: Option<Map<String, JsonValue>>,
    setglobal: Option<Map<String, JsonValue>>,
}

pub struct Vim;

impl Vim {
    fn pipes_dir(&self) -> PathBuf {
        [
            dirs::data_dir().unwrap().to_str().unwrap(),
            "thcon",
            "vim"
        ].iter().collect()
    }
}


impl Themeable for Vim {
    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let config = match &config.vim {
            Some(vim) => vim,
            None => {
                return Err(
                    Box::new(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            "Couldn't find [vim] section in thcon.toml"
                        )
                    )
                )
            }
        };

        let payload = match operation {
            Operation::Darken => &config.dark,
            Operation::Lighten => &config.light
        };
        let payload = serde_json::to_string(payload)? + "\n";


        let pipes_dir = self.pipes_dir();
        let pipes = match fs::read_dir(pipes_dir) {
            Ok(pipes) => Ok(Some(pipes)),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Ok(None),
                _ => Err(Box::new(e) as Box<dyn Error>)
            }
        };

        return pipes.map(|pipes| {
            match pipes {
                None => (),
                Some(pipes) => {
                    for pipe in pipes {
                        pipe.map(|pipe| {
                            fs::write(pipe.path(), &payload).unwrap_or(());
                        }).unwrap_or(());
                    }
                }
            }
        });
    }
}
