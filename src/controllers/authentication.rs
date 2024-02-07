use axum::Json;

use crate::types::user::{NewUserDto, User};

pub async fn register(Json(payload): Json<NewUserDto>) -> Json<User> {
    println!("Hello");
    Json(User {
        id: 1,
        name: payload.name,
        surname: payload.surname,
        username: payload.username,
        email: payload.email,
        password: payload.password,
        created_at: chrono::Utc::now(),
    })
}
