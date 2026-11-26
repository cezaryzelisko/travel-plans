use serde::Serialize;

use crate::domain::tag::Tag;

#[derive(Serialize)]
pub struct TagDto {
    pub label: String,
    pub color: String,
}

impl From<&Tag> for TagDto {
    fn from(value: &Tag) -> Self {
        Self {
            label: value.label.clone(),
            color: value.color.to_string(),
        }
    }
}
