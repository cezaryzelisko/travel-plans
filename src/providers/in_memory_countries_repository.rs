use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard},
};

use async_trait::async_trait;

use crate::{
    domain::{country::Country, country_code::CountryCode},
    repositories::{countries_repository::CountriesRepository, repository_error::RepositoryError},
};

pub struct InMemoryCountriesRepository {
    countries: RwLock<HashMap<CountryCode, Country>>,
}

impl InMemoryCountriesRepository {
    pub fn new() -> Self {
        let poland = Country::new(
            "Poland".to_string(),
            CountryCode::parse("PL".to_string()).expect("hardcoded version of repo"),
        );
        let spain = Country::new(
            "Spain".to_string(),
            CountryCode::parse("ES".to_string()).expect("hardcoded version of repo"),
        );
        let portugal = Country::new(
            "Portugal".to_string(),
            CountryCode::parse("PT".to_string()).expect("hardcoded version of repo"),
        );

        Self {
            countries: RwLock::new(HashMap::from([
                (poland.code.clone(), poland),
                (spain.code.clone(), spain),
                (portugal.code.clone(), portugal),
            ])),
        }
    }

    fn read_countries(
        &self,
    ) -> Result<RwLockReadGuard<'_, HashMap<CountryCode, Country>>, RepositoryError> {
        self.countries
            .read()
            .map_err(|e| RepositoryError::UnknownState(e.to_string()))
    }
}

#[async_trait]
impl CountriesRepository for InMemoryCountriesRepository {
    async fn all(&self) -> Result<Vec<Country>, RepositoryError> {
        Ok(self.read_countries()?.values().cloned().collect())
    }

    async fn find_one_by_code(
        &self,
        code: &CountryCode,
    ) -> Result<Option<Country>, RepositoryError> {
        Ok(self.read_countries()?.get(code).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_all_seed_countries() {
        let repository = InMemoryCountriesRepository::new();
        let countries = repository.all().await;
        let countries: Vec<CountryCode> = countries
            .unwrap()
            .into_iter()
            .map(|country| country.code)
            .collect();

        assert_eq!(countries.len(), 3);
        assert!(
            countries.contains(&CountryCode::parse("ES".to_string()).unwrap()),
            "missing ES code"
        );
        assert!(
            countries.contains(&CountryCode::parse("PL".to_string()).unwrap()),
            "missing PL code"
        );
        assert!(
            countries.contains(&CountryCode::parse("PT".to_string()).unwrap()),
            "missing PT code"
        );
    }
}
