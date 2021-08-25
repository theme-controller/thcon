use anyhow::{anyhow, Result};
use log::{error, info};

use crate::app;
use crate::{Config, ConfigState, Operation};

pub fn switch(
    config: &Config,
    name: &str,
    was_requested: bool,
    operation: &Operation,
) -> Result<()> {
    let app = match app::get(name) {
        None => {
            return Ok(());
        }
        Some(app) => app,
    };

    match app.config_state(&config) {
        ConfigState::NoDefault => {
            if was_requested {
                error!(target: name, "skipping (needs manual configuration)");
                Err(anyhow!("skipping {} (needs manual configuration)", name))
            } else {
                info!(target: name, "skipping (needs manual configuration)");
                Ok(())
            }
        }
        ConfigState::Disabled => {
            info!(target: name, "skipping (disabled)");
            Ok(())
        }
        ConfigState::Default => {
            info!(target: name, "{}ing (default configuration)", operation);
            let res = app.switch(&config, &operation);
            if let Err(ref e) = res {
                error!(target: name, "{:#}", e);
            }
            res
        }
        ConfigState::Enabled => {
            info!(target: name, "{}ing", operation);
            let res = app.switch(&config, &operation);
            if let Err(ref e) = res {
                error!(target: name, "{:#}", e);
            }
            res
        }
    }
}
