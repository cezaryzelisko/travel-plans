use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::{pages::home::HomeTemplate, state::AppState};

pub async fn get_index_page(State(state): State<AppState>) -> impl IntoResponse {
    let countries = match state.countries_repository.all().await {
        Ok(result) => result,
        Err(_) => {
            return Html("Failed to get countries from store".to_string());
        }
    };
    let template_result = HomeTemplate::new(&countries).render();

    match template_result {
        Ok(template) => Html(template),
        Err(_) => Html("Failed to render template".to_string()),
    }
}
