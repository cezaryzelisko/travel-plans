use std::sync::Arc;

use crate::repositories::countries_repository::CountriesRepository;

#[derive(Clone)]
pub struct AppState {
    pub countries_repository: Arc<dyn CountriesRepository>,
}

impl AppState {
    pub fn new(countries_repository: Arc<dyn CountriesRepository>) -> Self {
        Self {
            countries_repository,
        }
    }
}
