use thiserror::Error;

#[derive(Error, Debug)]
pub enum HundError {
    #[error("I'm hungry! Woof!")]
    Hungry,
    #[error("Path already exists: {0}")]
    PathExists(String),
    #[error("Empty package name")]
    EmptyPackageName,
}
