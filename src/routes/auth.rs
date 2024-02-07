use axum::{routing::get, Router};

pub fn authentication() -> Router {
    Router::new().route("/auth", get(|| async { "Hello, World! Authentication" }))
}
