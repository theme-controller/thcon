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
//! Section: `vim` or `nvim`
//!
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
use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::{Value as JsonValue, Map};

use crate::themeable::Themeable;
use crate::operation::Operation;
use crate::config::Config as ThconConfig;
use crate::sockets;

#[derive(Debug, Deserialize)]
pub struct Config {
    dark: ConfigSection,
    light: ConfigSection,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigSection {
    colorscheme: Option<String>,
    r#let: Option<Map<String, JsonValue>>,
    set: Option<Map<String, JsonValue>>,
    setglobal: Option<Map<String, JsonValue>>,
}

impl ConfigSection {
    /// Renders a `ConfigSection` instance as a valid `vimrc` file, using vim-standard syntax.
    /// This works mostly because single-line JSON representations of non-booleans seem to be valid
    /// vimscript.
    fn to_vimrc(&self) -> String {
        let mut contents: Vec<String> = vec!();
        if let Some(sets) = &self.set {
            for (key, val) in sets.iter() {
                if val == true {
                    contents.push(format!("set {}", key));
                } else if val == false {
                    contents.push(format!("set no{}", key));
                } else {
                    let val: String = if let JsonValue::String(s) = val {
                        s.to_owned()
                    } else {
                        serde_json::to_string(val).unwrap()
                    };
                    contents.push(format!("set {}={}", key, &val));
                }
            }
        }

        if let Some(global_sets) = &self.setglobal {
            for (key, val) in global_sets.iter() {
                if val == true {
                    contents.push(format!("setglobal {}", key));
                } else if val == false {
                    contents.push(format!("setglobal no{}", key));
                } else {
                    let val: String = if let JsonValue::String(s) = val {
                        s.to_owned()
                    } else {
                        serde_json::to_string(val).unwrap()
                    };
                    contents.push(format!("setglobal {}={}", key, &val));
                }
            }
        }

        if let Some(lets) = &self.r#let {
            for (key, val) in lets.iter() {
                let val: String = if let JsonValue::String(s) = val {
                    // string values assigned to variables via `let` must be wrapped in quotes,
                    // or VimL treats them like variable names
                    format!(r#""{}""#, s)
                } else {
                    serde_json::to_string(val).unwrap()
                };
                contents.push(format!("let {}={}", key, &val));
            }
        }

        // set colorscheme last, to invoke any `autocmd`s triggered by the `ColorScheme` event with all
        // settings already available.
        if let Some(colorscheme) = &self.colorscheme {
            contents.push(format!("colorscheme {}", colorscheme));
        }

        contents.join("\n")
    }
}

#[derive(Debug, Serialize)]
struct WirePayload {
    rc_file: String,
}

/// A Thcon-controlled vim variant, e.g. vim or neovim.
trait ControlledVim {
    /// The name of the thcon.toml section to read.
    const SECTION_NAME: &'static str;
    /// The name of the vim variant's rc file.
    const RC_NAME: &'static str;
    /// Returns the path where thcon.vim's named pipes for this variant are stored.
    fn sock_dir() -> PathBuf {
        let addr = sockets::socket_addr(Self::SECTION_NAME, true);
        PathBuf::from(addr.parent().unwrap())
    }
    /// Returns an `Option<Config>` for this variant's parsed section in thcon.toml.
    fn extract_config(thcon_config: &ThconConfig) -> &Option<Config>;
}

pub struct Vim;
impl ControlledVim for Vim {
    const SECTION_NAME: &'static str = "vim";
    const RC_NAME: &'static str = "vimrc";
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
    const RC_NAME: &'static str = "nvimrc";
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

    let rc_dir = crate::dirs::data().unwrap().join("thcon/");
    if !rc_dir.exists() {
        fs::create_dir_all(&rc_dir)?;
    }

    let rc_path = &rc_dir.join(V::RC_NAME);
    fs::write(&rc_path, payload.to_vimrc()).unwrap();

    let wire_payload = WirePayload{ rc_file: rc_path.to_str().unwrap_or_default().to_string() };
    let wire_payload = serde_json::to_vec(&wire_payload).unwrap_or_default();

    let sock_dir = V::sock_dir();
    let sockets = match fs::read_dir(sock_dir) {
        Ok(sockets) => Ok(Some(sockets)),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Ok(None),
            _ => Err(Box::new(e) as Box<dyn Error>)
        }
    };

    sockets.map(|sockets| {
        match sockets {
            None => (),
            Some(sockets) => {
                for sock in sockets {
                    if sock.is_err() { continue; }
                    let sock = sock.unwrap().path();
                    if let Ok(mut stream) = std::os::unix::net::UnixStream::connect(&sock) {
                        stream.write_all(&wire_payload).unwrap_or(())
                    }
                }
            }
        }
    })
}


#[test]
fn to_vimrc_empty_input() {
    let config = ConfigSection::default();
    assert_eq!(
        config.to_vimrc(),
        "",
    );
}

#[test]
fn to_vimrc_all_sections() {
    use serde_json::json;

    let config = ConfigSection {
        set: Some([
                  ("background".to_string(), json!("dark")),
                  ("number".to_string(), json!(true)),
        ].iter().cloned().collect()),
        setglobal: Some([
                  ("tw".to_string(), json!(100)),
                  ("relnum".to_string(), json!(false)),
        ].iter().cloned().collect()),
        r#let: Some([
                  ("g:foo".to_string(), json!("new g:foo")),
                  ("bar".to_string(), json!(5)),
        ].iter().cloned().collect()),
        colorscheme: Some("shine".to_string()),
    };

    assert_eq!(
        config.to_vimrc(),
        vec!(
            "set background=dark",
            "set number",
            // `setglobal`s are slightly out of order because serde_json::Map isn't sorted.
            // In practice this is fine for now, but will eventually need to be addressed -
            // probably with a schema change in thcon.toml.
            "setglobal norelnum",
            "setglobal tw=100",
            // same for `let`s :(
            "let bar=5",
            r#"let g:foo="new g:foo""#,
            "colorscheme shine",
        ).join("\n"),
    );
}
