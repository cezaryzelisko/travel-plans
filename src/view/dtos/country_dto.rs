use serde::Serialize;

use crate::{domain::country::Country, view::dtos::tag_dto::TagDto};

#[derive(Serialize)]
pub struct CountryDto {
    pub name: String,
    pub code: String,
    pub tags: Vec<TagDto>,
}

impl From<&Country> for CountryDto {
    fn from(value: &Country) -> Self {
        Self {
            name: value.name.to_string(),
            code: value.code.to_string(),
            tags: value.tags.iter().map(TagDto::from).collect(),
        }
    }
}
