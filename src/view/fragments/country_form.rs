use askama::Template;

use crate::view::dtos::tag_dto::TagDto;

#[derive(Template)]
#[template(path = "fragments/country_form.html")]
pub struct CountryFormTemplate {
    operation: &'static str,
    country_code: String,
    country_name: String,
    tags: Vec<TagDto>,
}

impl CountryFormTemplate {
    pub fn new(
        operation: &'static str,
        country_code: String,
        country_name: String,
        tags: Vec<TagDto>,
    ) -> Self {
        Self {
            operation,
            country_code,
            country_name,
            tags,
        }
    }
}
