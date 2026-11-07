use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::repositories::repository_error::RepositoryError;

#[derive(Debug)]
pub enum AppError {
    Storage(RepositoryError),
    Template(askama::Error),
}

impl From<RepositoryError> for AppError {
    fn from(value: RepositoryError) -> Self {
        Self::Storage(value)
    }
}

impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        Self::Template(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("Error: {self:?}");

        match self {
            Self::Storage(_) | Self::Template(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
            }
        }
    }
}
