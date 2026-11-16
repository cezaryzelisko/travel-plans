use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};

use crate::{
    api::app_error::AppError, domain::country_code::CountryCode, state::AppState,
    view::fragments::country_details::CountryDetailsTemplate,
};

pub async fn get_country_details(
    State(state): State<AppState>,
    Path(country_code): Path<String>,
) -> Result<Html<String>, AppError> {
    let parsed_code = CountryCode::parse(country_code).map_err(AppError::InvalidInput)?;
    let country = state
        .countries_repository
        .find_one_by_code(&parsed_code)
        .await?
        .ok_or_else(|| AppError::EntityNotFound(parsed_code.to_string()))?;
    let template = CountryDetailsTemplate::new(&country).render()?;

    Ok(Html(template))
}
