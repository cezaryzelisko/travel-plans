use askama::Template;

use crate::{domain::country::Country, view::dtos::country_dto::CountryDto};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    countries: Vec<CountryDto>,
}

impl HomeTemplate {
    pub fn new(countries: &[Country]) -> Self {
        Self {
            countries: countries.iter().map(CountryDto::from).collect(),
        }
    }
}
