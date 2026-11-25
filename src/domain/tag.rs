use crate::domain::color::Color;

#[derive(Debug, Clone)]
pub struct Tag {
    pub label: String,
    pub color: Color,
}

impl Tag {
    pub fn new(label: String, color: Color) -> Self {
        Self { label, color }
    }
}
