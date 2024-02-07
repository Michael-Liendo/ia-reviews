use axum::{routing::post, Router};

use crate::controllers;

pub fn authentication() -> Router {
    Router::new().route("/register", post(controllers::authentication::register))
}
