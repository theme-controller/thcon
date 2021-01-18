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

trait ControlledVim {
    const SECTION_NAME: &'static str;
    fn pipes_dir() -> PathBuf {
        [
            dirs::data_dir().unwrap().to_str().unwrap(),
            "thcon",
            Self::SECTION_NAME
        ].iter().collect()
    }
    fn extract_config(thcon_config: &ThconConfig) -> &Option<Config>;
}

pub struct Vim;
impl ControlledVim for Vim {
    const SECTION_NAME: &'static str = "vim";
    fn extract_config(thcon_config: &ThconConfig) -> &Option<Config> {
        &thcon_config.vim
    }
}
impl Themeable for Vim {
    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        anyvim_switch::<Vim>(config, operation)
    }
}

pub struct Neovim;
impl ControlledVim for Neovim {
    const SECTION_NAME: &'static str = "nvim";
    fn extract_config(thcon_config: &ThconConfig) -> &Option<Config> {
        &thcon_config.nvim
    }
}
impl Themeable for Neovim {
    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        anyvim_switch::<Neovim>(config, operation)
    }
}

fn anyvim_switch<V: ControlledVim>(config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
    let config = match V::extract_config(config) {
        Some(section) => section,
        None => {
            return Err(
                Box::new(
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "Couldn't find [{}] section in thcon.toml",
                            V::SECTION_NAME
                        )
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


    let pipes_dir = V::pipes_dir();
    let pipes = match fs::read_dir(pipes_dir) {
        Ok(pipes) => Ok(Some(pipes)),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Ok(None),
            _ => Err(Box::new(e) as Box<dyn Error>)
        }
    };

    pipes.map(|pipes| {
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
    })
}