use serde::Serialize;

use crate::{domain::country::Country, view::dtos::tag_dto::TagDto};

#[derive(Serialize)]
pub struct CountryDto {
    pub name: String,
    pub code: String,
    pub is_visited: bool,
    pub tags: Vec<TagDto>,
}

impl From<&Country> for CountryDto {
    fn from(value: &Country) -> Self {
        Self {
            name: value.name.to_string(),
            code: value.code.to_string(),
            is_visited: value.is_visited,
            tags: value.tags.iter().map(TagDto::from).collect(),
        }
    }
}
