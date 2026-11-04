mod domain;
mod providers;

use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::{
    domain::country::Country,
    providers::in_memory_countries_repository::InMemoryCountriesRepository,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    countries: &'a [Country],
}

impl<'a> IndexTemplate<'a> {
    fn new(countries: &'a [Country]) -> Self {
        IndexTemplate { countries }
    }
}

async fn get_index_page() -> impl IntoResponse {
    let countries_repository = InMemoryCountriesRepository::new();
    let template_result = IndexTemplate::new(countries_repository.get_countries()).render();

    match template_result {
        Ok(template) => Html(template),
        Err(_) => Html("Failed to render template".to_string()),
    }
}

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:3000";
    let app = Router::new().route("/", get(get_index_page));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("App is running on: http://{}", address);

    axum::serve(listener, app).await.unwrap();
}
