use async_trait::async_trait;

use crate::{domain::country::Country, repositories::repository_error::RepositoryError};

#[async_trait]
pub trait CountriesRepository: Send + Sync {
    async fn all(&self) -> Result<Vec<Country>, RepositoryError>;
}
