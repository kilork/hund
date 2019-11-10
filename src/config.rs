use std::collections::HashMap;
use toml;
use serde::{Deserialize, Serialize};

pub (crate) struct HundSettings {
    download: Option<HundSettingsDownload>,
}

pub (crate) struct HundSettingsDownload {
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

pub(crate) struct HundAction {

}