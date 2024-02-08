use axum::{routing::post, Router};

use crate::{controllers, types::state::AppState};

pub fn authentication() -> Router<AppState> {
    Router::new().route("/register", post(controllers::authentication::register))
}
