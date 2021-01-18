//! Switches [vim](https://vim.org) and [Neovim](https://neovim.org) colorschemes (and other arbitrary settings)
//! 
//! ## Usage: Windows
//! Windows is not yet supported, but `vim`/`nvim` under WSL should work just fine.
//! 
//! ## Usage: macOS & Linux
//! Install [thcon.vim](https://github.com/sjbarag/thcon.vim) via your `.vimrc` or `init.vim`
//! according to its README, adding both the relevant line for your plugin manager and `call
//! thcon#listen()`.
//! 
//! In your `thcon.toml`, define light and dark themes. All values within 'dark' and 'light' are
//! optional (blank values cause no changes):
//! 
//! ```toml
//! [vim]
//! light.colorscheme = "shine"
//! dark.colorscheme = "blue"
//! 
//! [vim.light]
//! colorscheme = "shine"
//! 
//! [vim.light.set]
//! background = "light"
//! 
//! [vim.dark]
//! colorscheme = "blue"
//! 
//! [vim.dark.set]
//! background = "dark"
//! ```
//! 
//! or:
//! 
//! ```toml
//! [neovim]
//! dark.colorscheme = "default"
//! dark.set.background = "dark"
//! dark.let."g:lightline" = { colorscheme = "ayu_dark" }
//! light.colorscheme = "shine"
//! light.set.background = "light"
//! light.let."g:lightline" = { colorscheme = "ayu_light" }
//! ```
//! 
//! Feel free to use whichever syntax you prefer (or any other), as long as it's valid TOML.
//! 
//! ## `thcon.toml` Schema
//! | Key | Type | Description | Default |
//! | --- | ---- | ----------- | -------- |
//! | light | table | Settings to apply in light mode | (none) |
//! | light.colorscheme | string | The colorscheme to apply in light mode | (none) |
//! | light.set | table | Set of key/value pairs to apply with `:set` in light mode | (none) |
//! | light.setglobal | table | Set of key/value pairs to apply with `:setglobal` in light mode | (none) |
//! | light.let | table | Set of key/value pairs to apply with `:let` in light mode | (none) |
//! | dark | table | Settings to apply in dark mode | (none) |
//! | dark.colorscheme | string | The colorscheme to apply in dark mode | (none) |
//! | dark.set | table | Set of key/value pairs to apply with `:set` in dark mode | (none) |
//! | dark.setglobal | table | Set of key/value pairs to apply with `:setglobal` in dark mode | (none) |
//! | dark.let | table | Set of key/value pairs to apply with `:let` in dark mode | (none) |
//! 


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

/// A Thcon-controlled vim variant, e.g. vim or neovim.
trait ControlledVim {
    /// The name of the thcon.toml section to read.
    const SECTION_NAME: &'static str;
    /// Returns the path where thcon.vim's named pipes for this variant are stored.
    fn pipes_dir() -> PathBuf {
        [
            dirs::data_dir().unwrap().to_str().unwrap(),
            "thcon",
            Self::SECTION_NAME
        ].iter().collect()
    }
    /// Returns an `Option<Config>` for this variant's parsed section in thcon.toml.
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
    fn has_config(&self, config: &ThconConfig) -> bool {
        Vim::extract_config(config).is_some()
    }

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
    fn has_config(&self, config: &ThconConfig) -> bool {
        Neovim::extract_config(config).is_some()
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<(), Box<dyn Error>> {
        anyvim_switch::<Neovim>(config, operation)
    }
}

/// Switches settings and colorscheme in a `vim`-agnostic way.
/// Returns unit result if successful, otherwise the causing error.
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