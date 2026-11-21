use crate::domain::{country_code::CountryCode, country_name::CountryName};

#[derive(Debug, Clone)]
pub struct Country {
    pub name: CountryName,
    pub code: CountryCode,
}

impl Country {
    pub fn new(name: CountryName, code: CountryCode) -> Self {
        Self { name, code }
    }
}
