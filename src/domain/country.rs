use crate::domain::country_code::CountryCode;

pub struct Country {
    pub name: String,
    pub code: CountryCode,
}

impl Country {
    pub fn new(name: String, code: CountryCode) -> Self {
        Self { name, code }
    }
}
