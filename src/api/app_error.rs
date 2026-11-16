use std::error::Error;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    domain::country_code::CountryCodeError, repositories::repository_error::RepositoryError,
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("storage operation failed")]
    Storage(#[from] RepositoryError),
    #[error("failed to render the page template")]
    Template(#[from] askama::Error),
    #[error("invalid input provided")]
    InvalidInput(#[source] CountryCodeError),
    #[error("entity not found")]
    EntityNotFound(String),
}

impl AppError {
    fn log_errors(&self) {
        let mut source = self.source();
        eprintln!("error: {self}");

        while let Some(src) = source {
            source = src.source();
            eprintln!("caused by: {src}");
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Storage(_) | Self::Template(_) => {
                self.log_errors();
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
            }
            Self::InvalidInput(country_code_error) => {
                let message = format!("Provided invalid input: {}", country_code_error);
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            Self::EntityNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Entity identified by {} not found", id),
            )
                .into_response(),
        }
    }
}
