use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountryName(String);

impl CountryName {
    pub fn parse(name: String) -> Result<Self, CountryNameError> {
        let trimmed_name = name.trim();

        if trimmed_name.is_empty() {
            return Err(CountryNameError::InvalidFormat);
        }

        Ok(Self(trimmed_name.to_string()))
    }
}

impl Display for CountryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CountryNameError {
    #[error("provided country name has invalid format, it cannot be empty")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_country_name() {
        let country_name_result = CountryName::parse("".to_string());
        assert!(matches!(
            country_name_result,
            Err(CountryNameError::InvalidFormat)
        ))
    }

    #[test]
    fn rejects_whitespace_only_country_name() {
        let country_name_result = CountryName::parse("    ".to_string());
        assert!(matches!(
            country_name_result,
            Err(CountryNameError::InvalidFormat)
        ))
    }

    #[test]
    fn accepts_valid_country_name() {
        let country_name = CountryName::parse("Poland".to_string());
        assert!(country_name.is_ok())
    }

    #[test]
    fn trims_country_name() {
        let country_name = CountryName::parse(" Poland ".to_string()).unwrap();
        assert_eq!(country_name.to_string(), "Poland".to_string())
    }
}
