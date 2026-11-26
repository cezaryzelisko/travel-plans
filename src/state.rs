use std::sync::Arc;

use crate::repositories::{
    countries_repository::CountriesRepository, tags_repository::TagsRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub countries_repository: Arc<dyn CountriesRepository>,
    pub tags_repository: Arc<dyn TagsRepository>,
}

impl AppState {
    pub fn new(
        countries_repository: Arc<dyn CountriesRepository>,
        tags_repository: Arc<dyn TagsRepository>,
    ) -> Self {
        Self {
            countries_repository,
            tags_repository,
        }
    }
}
