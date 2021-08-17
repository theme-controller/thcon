//! Switches between Light and Dark themes for Apps and the Windows System.

use crate::themeable::{ConfigState, Themeable};
use crate::operation::Operation;
use crate::config::Config as ThconConfig;
use crate::Disableable;
use crate::AppConfig;

use anyhow::{Context, Result};
use log::{warn,error};
use winreg::RegKey;
use winreg::transaction::Transaction;

use std::error::Error;
use std::io;

use serde::Deserialize;

#[derive(Debug, Deserialize, Disableable, AppConfig)]
pub struct _Config {
    #[serde(default)]
    disabled: bool,
}

pub struct Windows;

impl Themeable for Windows {
    fn config_state(&self, config: &ThconConfig) -> ConfigState {
        ConfigState::with_default_config(config.windows.as_ref().map(|c| c.inner.as_ref()))
    }

    fn switch(&self, config: &ThconConfig, operation: &Operation) -> Result<()> {
        let should_use_light_theme: u32 = match operation {
            Operation::Lighten => 1,
            Operation::Darken => 0,
        };

        match Transaction::new() {
            Err(e) => {
                error!("Unable to create Windows registry transaction: {}", e);
                Err(e.into())
            },
            Ok(t) => {
                let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
                let section = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
                let personalize = hkcu.open_subkey_transacted_with_flags(section, &t, winreg::enums::KEY_ALL_ACCESS)
                    .with_context(|| format!("Unable to find registry section '{}'", section))?;

                let _: u32 = personalize.get_value("AppsUseLightTheme")
                        .context("Unable to find setting 'AppsUseLightTheme'")?;
                
                let _: u32 = personalize.get_value("SystemUsesLightTheme")
                        .context("Unable to find setting 'SystemUsesLightTheme'")?;
                
                personalize.set_value("AppsUseLightTheme", &should_use_light_theme)
                    .map(|_| personalize.set_value("SystemUsesLightTheme", &should_use_light_theme))
                    .map(|_| t.commit() )
                    .map(|_| Ok(()))
                    .context("Unable to save registry changes")?
            }
        }
    }
}