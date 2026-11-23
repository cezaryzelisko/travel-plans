use askama::Template;
use axum::{Form, extract::State, http::StatusCode, response::Html};

use crate::{
    api::app_error::AppError,
    domain::{country::Country, country_code::CountryCode, country_name::CountryName},
    state::AppState,
    view::{
        dtos::country_create_dto::CountryCreateDto,
        fragments::country_details::CountryDetailsTemplate,
    },
};

pub async fn create_country(
    State(state): State<AppState>,
    Form(form): Form<CountryCreateDto>,
) -> Result<(StatusCode, Html<String>), AppError> {
    let country_name =
        CountryName::parse(form.name).map_err(|e| AppError::InvalidInput(e.into()))?;
    let country_code =
        CountryCode::parse(form.code).map_err(|e| AppError::InvalidInput(e.into()))?;
    let country = Country::new(country_name, country_code);
    let country = state.countries_repository.create(country).await?;
    let template = CountryDetailsTemplate::new(&country).render()?;

    Ok((StatusCode::CREATED, Html(template)))
}
