use crate::{
    error::Errors,
    services,
    types::{
        state::AppState,
        user::{NewUserDto, User},
    },
};
use anyhow::Result;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<NewUserDto>,
) -> Result<impl IntoResponse, Errors> {
    // todo: implement service and repository functionality
    services::User::create_user(&state.conn, payload).await?;
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
