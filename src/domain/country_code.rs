use std::fmt::Display;

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

impl Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CountryCodeError {
    #[error("invalid code length, got {0} chars, expected {expected}", expected = CountryCode::CODE_LENGTH)]
    InvalidLength(usize),
    #[error("invalid code format, only A-Z letters are accepted")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_uppercase_code() {
        let result = CountryCode::parse("PL".to_string());
        assert!(result.is_ok())
    }

    #[test]
    fn accepts_valid_lowercase_code() {
        let result = CountryCode::parse("pl".to_string());
        assert!(result.is_ok())
    }

    #[test]
    fn transforms_valid_lowercase_code_to_uppercase() {
        let result = CountryCode::parse("pl".to_string());
        assert_eq!(result.unwrap().0, "PL")
    }

    #[test]
    fn rejects_code_longer_than_2_letters() {
        let result = CountryCode::parse("ABC".to_string());
        assert!(matches!(result, Err(CountryCodeError::InvalidLength(3))))
    }

    #[test]
    fn rejects_code_shorter_than_2_letters() {
        let result = CountryCode::parse("A".to_string());
        assert!(matches!(result, Err(CountryCodeError::InvalidLength(1))))
    }

    #[test]
    fn rejects_empty_code() {
        let result = CountryCode::parse("".to_string());
        assert!(matches!(result, Err(CountryCodeError::InvalidLength(0))))
    }

    #[test]
    fn rejects_code_with_digit() {
        let result = CountryCode::parse("A1".to_string());
        assert!(matches!(result, Err(CountryCodeError::InvalidFormat)))
    }

    #[test]
    fn rejects_code_with_symbol() {
        let result = CountryCode::parse("A!".to_string());
        assert!(matches!(result, Err(CountryCodeError::InvalidFormat)))
    }

    #[test]
    fn rejects_code_with_non_ascii_letter() {
        let result = CountryCode::parse("AĄ".to_string());
        assert!(matches!(result, Err(CountryCodeError::InvalidFormat)))
    }

    #[test]
    fn prints_two_letter_representation() {
        let result = CountryCode::parse("pl".to_string());
        assert_eq!(result.unwrap().to_string(), "PL")
    }
}
