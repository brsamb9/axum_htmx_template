use std::sync::Arc;

use axum::{Router, Server};

use axum_htmx_base::{
    api::{get_api_router, AppState},
    init_tracing,
    pages::get_pages_router,
};

use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    info!("initializing router, database and assets");

    let shared_state = Arc::new(AppState::new().await);

    let assets_path_dir = format!(
        "{}/assets",
        std::env::current_dir().unwrap().to_str().unwrap()
    );

    let app = Router::new()
        .merge(get_pages_router())
        .nest("/api", get_api_router(shared_state))
        .nest_service("/assets", ServeDir::new(assets_path_dir));

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    // Todo: shutdown database?
    info!("Shutting down server!")
}
