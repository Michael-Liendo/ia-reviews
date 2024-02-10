use crate::{
    error::Errors,
    types::user::{NewUserDto, User},
};
use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn register(Json(payload): Json<NewUserDto>) -> Result<impl IntoResponse, Errors> {
    // todo: implement service and repository functionality
    Ok((
        StatusCode::OK,
        Json(User {
            id: 32,
            name: "Michael".to_string(),
            surname: "Liendo".to_string(),
            email: "michael".to_string(),
            password_hash: "hello".to_string(),
            created_at: chrono::Utc::now(),
        }),
    ))
}
