#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("cannot read data from the store")]
    Read,
}
