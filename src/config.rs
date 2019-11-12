use crate::HUND_TOML;
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
        let hund_settings_path = dirs::home_dir()
            .unwrap_or_default()
            .join(crate::HUND_SETTINGS)
            .join(crate::HUND_CONFIG);
        if hund_settings_path.is_file() {
            match fs::read_to_string(&hund_settings_path) {
                Ok(hund_settings) => match toml::from_str(&hund_settings) {
                    Ok(hund_settings) => hund_settings,
                    Err(e) => {
                        eprintln!("Cannot parse {:?}: {}", hund_settings_path, e);
                        Default::default()
                    }
                },
                Err(e) => {
                    eprintln!("Cannot read {:?}: {}", hund_settings_path, e);
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
    pub(crate) buffer_size: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct HundConfig {
    pub(crate) project: HundProject,
    pub(crate) dependencies: HashMap<String, String>,
    pub(crate) actions: HashMap<String, HundAction>,
}

impl HundConfig {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            project: HundProject {
                name: name.into(),
                version: "0.0.1".into(),
                authors: vec![],
            },
            dependencies: HashMap::new(),
            actions: HashMap::new(),
        }
    }

    pub(crate) fn load() -> Result<Self, failure::Error> {
        Ok(toml::from_str(&fs::read_to_string(HUND_TOML)?)?)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct HundProject {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) authors: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct HundAction {}
