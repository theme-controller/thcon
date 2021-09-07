use std::time::Instant;

use anyhow::{anyhow, Result};
use log::{error, info, trace};

use crate::app;
use crate::{Config, ConfigState, Operation};

pub fn switch(
    config: &Config,
    name: &str,
    was_requested: bool,
    operation: &Operation,
) -> Result<()> {
    let start = Instant::now();
    let app = match app::get(name) {
        None => {
            return Ok(());
        }
        Some(app) => app,
    };

    match app.config_state(config) {
        ConfigState::NoDefault => {
            if was_requested {
                error!(target: name, "skipping (needs manual configuration)");
                trace!(
                    target: name,
                    "completed in {} ms",
                    (Instant::now() - start).as_millis()
                );
                Err(anyhow!("skipping {} (needs manual configuration)", name))
            } else {
                info!(target: name, "skipping (needs manual configuration)");
                trace!(
                    target: name,
                    "completed in {} ms",
                    (Instant::now() - start).as_millis()
                );
                Ok(())
            }
        }
        ConfigState::Disabled => {
            info!(target: name, "skipping (disabled)");
            trace!(
                target: name,
                "completed in {} ms",
                (Instant::now() - start).as_millis()
            );
            Ok(())
        }
        ConfigState::Default => {
            info!(target: name, "{}ing (default configuration)", operation);
            let res = app.switch(config, operation);
            if let Err(ref e) = res {
                error!(target: name, "{:#}", e);
            }
            trace!(
                target: name,
                "completed in {} ms",
                (Instant::now() - start).as_millis()
            );
            res
        }
        ConfigState::Enabled => {
            info!(target: name, "{}ing", operation);
            let res = app.switch(config, operation);
            if let Err(ref e) = res {
                error!(target: name, "{:#}", e);
            }
            trace!(
                target: name,
                "completed in {} ms",
                (Instant::now() - start).as_millis()
            );
            res
        }
    }
}
