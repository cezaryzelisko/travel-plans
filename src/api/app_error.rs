use std::error::Error;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::repositories::repository_error::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("storage operation failed")]
    Storage(#[from] RepositoryError),
    #[error("failed to render the page template")]
    Template(#[from] askama::Error),
    #[error("invalid input provided")]
    InvalidInput(#[source] Box<dyn Error + Send + Sync>),
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
        match &self {
            Self::Storage(RepositoryError::AlreadyExists(id)) => (
                StatusCode::CONFLICT,
                format!("Entity with id {} already exists", id),
            )
                .into_response(),
            Self::Storage(RepositoryError::UnknownState(_)) | Self::Template(_) => {
                self.log_errors();
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
            }
            Self::InvalidInput(invalid_input_error) => {
                let message = format!("Provided invalid input: {}", invalid_input_error);
                (StatusCode::BAD_REQUEST, message).into_response()
            }
        }
    }
}
