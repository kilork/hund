use crate::app::HundApp;
use crate::config::HundConfig;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{HundError, HUND_SETTINGS, HUND_TOML};

pub fn publish() -> Result<(), failure::Error> {
    let _ = HundApp::new();

    let hund_config = HundConfig::load()?;
    let name = &hund_config.project.name;

    let path = dirs::home_dir()
        .unwrap_or_default()
        .join(HUND_SETTINGS)
        .join(index_path(&name)?);

    dbg!(&path);
    fs::create_dir_all(&path)?;

    fs::copy(HUND_TOML, path.join(&hund_config.project.version))?;

    Ok(())
}

fn index_path(name: &str) -> Result<PathBuf, HundError> {
    Ok(match name.len() {
        0 => return Err(HundError::EmptyPackageName),
        1 => PathBuf::from("1"),
        2 => PathBuf::from("2"),
        3 => PathBuf::from("3").join(&name[..2]),
        _ => PathBuf::from(&name[..2]).join(&name[2..4]),
    }
    .join(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_path_checks() {
        assert_eq!(index_path("a").unwrap(), PathBuf::from("1").join("a"));
        assert_eq!(index_path("ab").unwrap(), PathBuf::from("2").join("ab"));
        assert_eq!(
            index_path("abc").unwrap(),
            PathBuf::from("3").join("ab").join("abc")
        );
        assert_eq!(
            index_path("abcd").unwrap(),
            PathBuf::from("ab").join("cd").join("abcd")
        );
        assert_eq!(
            index_path("abcde").unwrap(),
            PathBuf::from("ab").join("cd").join("abcde")
        );
    }
}
