use askama::Template;

use crate::{domain::country::Country, view::dtos::country_dto::CountryDto};

#[derive(Template)]
#[template(path = "fragments/country_details.html")]
pub struct CountryDetailsTemplate {
    country: CountryDto,
}

impl CountryDetailsTemplate {
    pub fn new(country: &Country) -> Self {
        Self {
            country: CountryDto::from(country),
        }
    }
}
