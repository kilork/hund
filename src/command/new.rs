use crate::app::HundApp;
use crate::config::HundConfig;
use std::fs;
use std::path::PathBuf;

use crate::HundError;

pub fn new(name: &str) -> Result<(), failure::Error> {
    let _ = HundApp::new();

    let path = PathBuf::from(name);

    if path.exists() {
        return Err(HundError::PathExists(name.into()).into());
    }

    fs::create_dir(&path)?;

    let hund_config = HundConfig::new(name);

    let config_str = toml::to_string(&hund_config)?;

    fs::write(path.join("hund.toml"), config_str)?;

    Ok(())
}
