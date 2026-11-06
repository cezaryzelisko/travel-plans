use async_trait::async_trait;

use crate::{
    domain::{country::Country, country_code::CountryCode},
    repositories::{countries_repository::CountriesRepository, repository_error::RepositoryError},
};

pub struct InMemoryCountriesRepository {
    countries: Vec<Country>,
}

impl InMemoryCountriesRepository {
    pub fn new() -> Self {
        Self {
            countries: vec![
                Country::new(
                    "Poland".to_string(),
                    CountryCode::parse("PL".to_string()).expect("hardcoded version of repo"),
                ),
                Country::new(
                    "Spain".to_string(),
                    CountryCode::parse("ES".to_string()).expect("hardcoded version of repo"),
                ),
                Country::new(
                    "Portugal".to_string(),
                    CountryCode::parse("PT".to_string()).expect("hardcoded version of repo"),
                ),
            ],
        }
    }
}

#[async_trait]
impl CountriesRepository for InMemoryCountriesRepository {
    async fn all(&self) -> Result<Vec<Country>, RepositoryError> {
        Ok(self.countries.clone())
    }
}
