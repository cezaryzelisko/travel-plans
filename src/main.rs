mod domain;
mod handlers;
mod pages;
mod providers;
mod repositories;
mod state;

use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::{
    handlers::home::get_home_page,
    providers::in_memory_countries_repository::InMemoryCountriesRepository, state::AppState,
};

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:3000";
    let state = AppState::new(Arc::new(InMemoryCountriesRepository::new()));

    let app = Router::new()
        .route("/", get(get_home_page))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to the address");
    println!("App is running on: http://{}", address);

    axum::serve(listener, app)
        .await
        .expect("Failed to serve the app");
}
