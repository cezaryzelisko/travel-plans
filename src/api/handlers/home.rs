use askama::Template;
use axum::{extract::State, response::Html};
use tokio::try_join;

use crate::{api::app_error::AppError, state::AppState, view::pages::home::HomeTemplate};

pub async fn get_home_page(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let (countries, tags) = try_join!(
        state.countries_repository.all(),
        state.tags_repository.all()
    )?;
    let template = HomeTemplate::new(&countries, &tags).render()?;

    Ok(Html(template))
}
