#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("data store is in an unknown state: {0}")]
    UnknownState(String),
}
