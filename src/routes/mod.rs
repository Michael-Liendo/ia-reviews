use axum::Router;

use crate::types::state::AppState;

pub mod auth;

pub fn create_routes() -> Router<AppState> {
    Router::new().nest("/auth", auth::authentication())
}
