use askama::Template;
use axum::{extract::State, response::Html};

use crate::{api::app_error::AppError, state::AppState, view::pages::home::HomeTemplate};

pub async fn get_home_page(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let countries = state.countries_repository.all().await?;
    let template = HomeTemplate::new(&countries).render()?;

    Ok(Html(template))
}
