use std::{
    collections::{HashMap, hash_map::Entry},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use async_trait::async_trait;

use crate::{
    domain::{country::Country, country_code::CountryCode, country_name::CountryName},
    repositories::{countries_repository::CountriesRepository, repository_error::RepositoryError},
};

pub struct InMemoryCountriesRepository {
    countries: RwLock<HashMap<CountryCode, Country>>,
}

impl InMemoryCountriesRepository {
    const REPO_PARSE_MESSAGE: &str = "hardcoded version of repo";

    pub fn new() -> Self {
        let poland = Country::new(
            CountryName::parse("Poland".to_string()).expect(Self::REPO_PARSE_MESSAGE),
            CountryCode::parse("PL".to_string()).expect(Self::REPO_PARSE_MESSAGE),
        );
        let spain = Country::new(
            CountryName::parse("Spain".to_string()).expect(Self::REPO_PARSE_MESSAGE),
            CountryCode::parse("ES".to_string()).expect(Self::REPO_PARSE_MESSAGE),
        );
        let portugal = Country::new(
            CountryName::parse("Portugal".to_string()).expect(Self::REPO_PARSE_MESSAGE),
            CountryCode::parse("PT".to_string()).expect(Self::REPO_PARSE_MESSAGE),
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

    fn write_countries(
        &self,
    ) -> Result<RwLockWriteGuard<'_, HashMap<CountryCode, Country>>, RepositoryError> {
        self.countries
            .write()
            .map_err(|e| RepositoryError::UnknownState(e.to_string()))
    }
}

#[async_trait]
impl CountriesRepository for InMemoryCountriesRepository {
    async fn all(&self) -> Result<Vec<Country>, RepositoryError> {
        Ok(self.read_countries()?.values().cloned().collect())
    }

    async fn create(&self, country: Country) -> Result<Country, RepositoryError> {
        match self.write_countries()?.entry(country.code.clone()) {
            Entry::Occupied(_) => Err(RepositoryError::AlreadyExists(country.code.to_string())),
            Entry::Vacant(slot) => Ok(slot.insert(country).clone()),
        }
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

    #[tokio::test]
    async fn adds_new_country() {
        let repository = InMemoryCountriesRepository::new();
        let country_code = CountryCode::parse("it".to_string()).unwrap();

        let write_country = repository
            .create(Country::new(
                CountryName::parse("Italy".to_string()).unwrap(),
                country_code.clone(),
            ))
            .await
            .unwrap();
        let find_country = repository
            .find_one_by_code(&country_code)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(write_country.code, country_code);
        assert_eq!(write_country.code, find_country.code);
    }

    #[tokio::test]
    async fn rejects_creation_of_existing_country() {
        let repository = InMemoryCountriesRepository::new();
        let country_code = CountryCode::parse("it".to_string()).unwrap();

        let write_country = repository
            .create(Country::new(
                CountryName::parse("Italy".to_string()).unwrap(),
                country_code.clone(),
            ))
            .await
            .unwrap();
        let duplicated_country_result = repository
            .create(Country::new(
                CountryName::parse("Country of Italy".to_string()).unwrap(),
                country_code.clone(),
            ))
            .await;
        let find_country = repository
            .find_one_by_code(&country_code)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(write_country.code, country_code);
        assert!(matches!(
            duplicated_country_result,
            Err(RepositoryError::AlreadyExists(_))
        ));
        assert_eq!(find_country.code, country_code);
        assert_eq!(find_country.name, write_country.name);
    }
}
