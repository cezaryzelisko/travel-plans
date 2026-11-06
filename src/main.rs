mod domain;
mod providers;
mod repositories;

use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_http::services::ServeDir;

use crate::{
    domain::country::Country,
    providers::in_memory_countries_repository::InMemoryCountriesRepository,
    repositories::countries_repository::CountriesRepository,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    countries: &'a [Country],
}

impl<'a> IndexTemplate<'a> {
    fn new(countries: &'a [Country]) -> Self {
        Self { countries }
    }
}

async fn get_index_page() -> impl IntoResponse {
    let countries_repository = InMemoryCountriesRepository::new();
    let countries = match countries_repository.all().await {
        Ok(result) => result,
        Err(_) => {
            return Html("Failed to get countries from store".to_string());
        }
    };
    let template_result = IndexTemplate::new(&countries).render();

    match template_result {
        Ok(template) => Html(template),
        Err(_) => Html("Failed to render template".to_string()),
    }
}

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:3000";
    let app = Router::new()
        .route("/", get(get_index_page))
        .nest_service("/public", ServeDir::new("public"));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("App is running on: http://{}", address);

    axum::serve(listener, app).await.unwrap();
}
