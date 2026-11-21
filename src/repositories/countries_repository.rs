use async_trait::async_trait;

use crate::{
    domain::{country::Country, country_code::CountryCode},
    repositories::repository_error::RepositoryError,
};

#[async_trait]
pub trait CountriesRepository: Send + Sync {
    async fn all(&self) -> Result<Vec<Country>, RepositoryError>;
    async fn create(&self, country: Country) -> Result<Country, RepositoryError>;
    async fn find_one_by_code(
        &self,
        code: &CountryCode,
    ) -> Result<Option<Country>, RepositoryError>;
}
