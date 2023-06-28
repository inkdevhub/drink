use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to build storage: {0}")]
    StorageBuilding(String),
}
