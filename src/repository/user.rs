use crate::types::user::User as UserType;
use ::entity::user::Entity as UserEntity;
use sea_orm::DbConn;

pub struct User {
    mutation: Mutation,
}

struct Mutation;

impl Mutation {
    pub fn create_user(db: &DbConn, user: UserEntity) -> UserType {
        todo!("Implement the create_user method")
    }
}
