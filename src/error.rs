use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::error::DbErr),

    #[error(transparent)]
    SerdeJSON(#[from] serde_json::Error),
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        error!("Error: {:#?}", self);

        match self {
            Self::DatabaseError(e) => {
                error!("Database Error: {:#?}", e);
                (
                    StatusCode::CONFLICT,
                    format!("Error executing a database query"),
                )
                    .into_response()
            }
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something happens {:#?}", e),
            )
                .into_response(),
        }
    }
}

trait ToResponse {
    fn to_response(&self) -> Response;
}
