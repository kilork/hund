use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml;

#[derive(Default, Deserialize)]
pub(crate) struct HundSettings {
    download: Option<HundSettingsDownload>,
}

impl HundSettings {
    pub(crate) fn load() -> Self {
        let hund_settings_path = dirs::home_dir().unwrap_or_default().join(crate::SETTINGS);
        if hund_settings_path.is_file() {
            match fs::read_to_string(&hund_settings_path) {
                Ok(hund_settings) => match toml::from_str(&hund_settings) {
                    Ok(hund_settings) => hund_settings,
                    Err(e) => {
                        eprintln!("Cannot parse {:?}: {}", hund_settings_path, e);
                        Default::default()
                    }
                }
                Err(e) => {
                    Default::default()
                }
            }
        } else {
            Default::default()
        }
    }
}

#[derive(Default, Deserialize)]
pub(crate) struct HundSettingsDownload {
    buffer_size: Option<usize>,
}

pub(crate) struct HundConfig {
    project: HundProject,
    dependencies: HashMap<String, String>,
    actions: HashMap<String, HundAction>,
}

pub(crate) struct HundProject {
    name: String,
    version: String,
    authors: Vec<String>,
}

pub(crate) struct HundAction {}
