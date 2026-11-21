use serde::Serialize;

use crate::domain::country::Country;

#[derive(Serialize)]
pub struct CountryDto {
    pub name: String,
    pub code: String,
}

impl From<&Country> for CountryDto {
    fn from(value: &Country) -> Self {
        Self {
            name: value.name.to_string(),
            code: value.code.to_string(),
        }
    }
}
