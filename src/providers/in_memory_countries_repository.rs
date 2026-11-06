use crate::domain::{country::Country, country_code::CountryCode};

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

impl InMemoryCountriesRepository {
    pub fn get_countries(&self) -> &[Country] {
        &self.countries
    }
}
