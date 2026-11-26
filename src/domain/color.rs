use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Color(String);

impl Color {
    pub const VALID_COLOR_LENGTHS: &[usize; 2] = &[4, 7];

    pub fn parse(color: String) -> Result<Self, ColorError> {
        if !color.starts_with('#') {
            return Err(ColorError::InvalidFormat);
        } else if !Self::VALID_COLOR_LENGTHS.contains(&color.len()) {
            return Err(ColorError::InvalidLength(color.len()));
        }

        Ok(Self(color))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("color has to start with '#'")]
    InvalidFormat,
    #[error("color has to be one of the following lengths (including '#' sign): {valid_lengths:?}", valid_lengths=Color::VALID_COLOR_LENGTHS)]
    InvalidLength(usize),
}
