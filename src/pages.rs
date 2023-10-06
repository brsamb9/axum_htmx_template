pub mod home_page;
pub mod page_utils;

use axum::{routing::get, Router};
pub use home_page::home_page;

pub fn get_pages_router() -> Router {
    Router::new().route("/", get(home_page))
}
