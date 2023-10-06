pub mod api;
pub mod db_client;
pub mod pages;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_htmx_base=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
