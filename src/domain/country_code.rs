#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CountryCode(String);

impl CountryCode {
    const CODE_LENGTH: usize = 2;

    pub fn parse(mut code: String) -> Result<Self, CountryCodeError> {
        let length = code.chars().count();

        if length != Self::CODE_LENGTH {
            return Err(CountryCodeError::InvalidLength(length));
        } else if code.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err(CountryCodeError::InvalidFormat);
        }

        code.make_ascii_uppercase();

        Ok(Self(code))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CountryCodeError {
    #[error("invalid code length, got {0} chars, expected {expected}", expected = CountryCode::CODE_LENGTH)]
    InvalidLength(usize),
    #[error("invalid code format, only A-Z letters are accepted")]
    InvalidFormat,
}
