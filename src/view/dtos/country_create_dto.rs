use serde::Deserialize;

#[derive(Deserialize)]
pub struct CountryCreateDto {
    pub name: String,
    pub code: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
