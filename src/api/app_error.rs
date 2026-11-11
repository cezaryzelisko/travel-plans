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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let mut source = self.source();
        eprintln!("error: {self}");

        while let Some(src) = source {
            source = src.source();
            eprintln!("caused by: {src}");
        }

        match self {
            Self::Storage(_) | Self::Template(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
            }
        }
    }
}
