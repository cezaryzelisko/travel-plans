use askama::Template;

use crate::{
    domain::{country::Country, tag::Tag},
    view::dtos::{country_dto::CountryDto, tag_dto::TagDto},
};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    countries: Vec<CountryDto>,
    tags: Vec<TagDto>,
}

impl HomeTemplate {
    pub fn new(countries: &[Country], tags: &[Tag]) -> Self {
        Self {
            countries: countries.iter().map(CountryDto::from).collect(),
            tags: tags.iter().map(TagDto::from).collect(),
        }
    }
}
