use serde::Deserialize;

#[derive(Deserialize)]
pub struct CountryQueryDto {
    pub name: Option<String>,
}
