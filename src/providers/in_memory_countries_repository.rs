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
