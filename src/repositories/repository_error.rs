#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("data store is in an unknown state: {0}")]
    UnknownState(String),
    #[error("provided entity {0} already exists in the data store")]
    AlreadyExists(String),
}
