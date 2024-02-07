use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct NewUserDto {
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}
