use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    NotFound,
    Database(sqlx::Error),
    Unauthorized,
    Internal,
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound,
            error => Self::Database(error),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "URL Not Found"),
            Self::Database(error) => {
                eprintln!("Database error: {:?}", error);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Ivalid email or Password"),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        (
            status,
            Json(json!({
                "error":message
            })),
        )
            .into_response()
    }
}
