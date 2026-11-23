use serde::Serialize;

#[derive(Serialize)]
pub struct CountryCodesDto {
    countries_updated: Vec<String>,
}

impl CountryCodesDto {
    pub fn new(codes: Vec<String>) -> Self {
        Self {
            countries_updated: codes,
        }
    }
}
