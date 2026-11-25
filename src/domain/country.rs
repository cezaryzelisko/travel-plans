use crate::domain::{country_code::CountryCode, country_name::CountryName, tag::Tag};

#[derive(Debug, Clone)]
pub struct Country {
    pub name: CountryName,
    pub code: CountryCode,
    pub is_visited: bool,
    pub tags: Vec<Tag>,
}

impl Country {
    pub fn new(
        name: CountryName,
        code: CountryCode,
        is_visited: Option<bool>,
        tags: Option<Vec<Tag>>,
    ) -> Self {
        Self {
            name,
            code,
            is_visited: is_visited.unwrap_or(false),
            tags: tags.unwrap_or(vec![]),
        }
    }
}
