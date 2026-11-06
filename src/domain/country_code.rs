#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CountryCode(String);

impl CountryCode {
    pub fn parse(mut code: String) -> Result<Self, CountryCodeError> {
        let length = code.chars().count();

        if length != 2 {
            return Err(CountryCodeError::InvalidLength(length));
        } else if code.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err(CountryCodeError::InvalidFormat);
        }

        code.make_ascii_uppercase();

        Ok(Self(code))
    }
}

#[derive(Debug)]
pub enum CountryCodeError {
    InvalidLength(usize),
    InvalidFormat,
}
