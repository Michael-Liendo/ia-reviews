use sea_orm::DbConn;

use crate::{
    error::Errors,
    repository,
    types::user::{NewUserDto, User as UserType},
};

// make user create services
pub struct User;

impl User {
    pub async fn create_user(db: &DbConn, dto: NewUserDto) -> Result<User, Errors> {
        let user = UserType {
            id: 0,
            name: dto.name,
            surname: dto.surname,
            email: dto.email,
            password_hash: dto.password,
            created_at: chrono::Utc::now(),
        };
        repository::User::create_user(db, user).await?;
        todo!("Implement user creation service")
    }
}
