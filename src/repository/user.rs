use crate::types::user::User as UserType;
use ::entity::user as UserEntity;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

pub struct User {
    mutation: Mutation,
}

struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        user: UserType,
    ) -> Result<UserEntity::ActiveModel, DbErr> {
        UserEntity::ActiveModel {
            name: Set(user.name),
            surname: Set(user.surname),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
