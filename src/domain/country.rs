use crate::domain::country_code::CountryCode;

#[derive(Debug, Clone)]
pub struct Country {
    pub name: String,
    pub code: CountryCode,
}

impl Country {
    pub fn new(name: String, code: CountryCode) -> Self {
        Self { name, code }
    }
}
