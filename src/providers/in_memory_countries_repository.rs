use crate::domain::country::Country;

pub struct InMemoryCountriesRepository {
    countries: Vec<Country>,
}

impl InMemoryCountriesRepository {
    pub fn new() -> Self {
        Self {
            countries: vec![
                Country::new("Poland".to_string()),
                Country::new("Spain".to_string()),
                Country::new("Portugal".to_string()),
            ],
        }
    }
}

impl InMemoryCountriesRepository {
    pub fn get_countries(&self) -> &[Country] {
        &self.countries
    }
}
