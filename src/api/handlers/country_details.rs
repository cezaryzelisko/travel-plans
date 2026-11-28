use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Html,
};

use crate::{
    api::app_error::AppError,
    domain::country_code::CountryCode,
    state::AppState,
    view::{
        dtos::{country_query_dto::CountryQueryDto, tag_dto::TagDto},
        fragments::{country_details::CountryDetailsTemplate, country_form::CountryFormTemplate},
    },
};

pub async fn get_country_details(
    State(state): State<AppState>,
    Path(country_code): Path<String>,
    Query(query): Query<CountryQueryDto>,
) -> Result<(StatusCode, Html<String>), AppError> {
    let parsed_code =
        CountryCode::parse(country_code).map_err(|e| AppError::InvalidInput(e.into()))?;
    let country = state
        .countries_repository
        .find_one_by_code(&parsed_code)
        .await?;

    let (status, template) = match country {
        Some(c) => (StatusCode::OK, CountryDetailsTemplate::new(&c).render()?),
        None => {
            let tags = state
                .tags_repository
                .all()
                .await?
                .iter()
                .map(TagDto::from)
                .collect();
            let name = query.name.unwrap_or_default();
            (
                StatusCode::NOT_FOUND,
                CountryFormTemplate::new("Create", parsed_code.to_string(), name, tags).render()?,
            )
        }
    };

    Ok((status, Html(template)))
}
