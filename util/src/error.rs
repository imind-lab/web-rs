use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database Error")]
    SqlxError(#[from] sqlx::Error),
    #[error("Unable to connect to the cache. ")]
    RedisError(#[from] redis::RedisError),

    #[error("Internal error: {0}")]
    Internal(String),
    #[error("{0} Not Found")]
    NotFound(String),
    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Template Error")]
    TemplateError(#[from] askama::Error),

    #[error("Contain Reserved Word: {0}")]
    ReservedWord(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::SqlxError(err) => {
                let msg = match err {
                    sqlx::Error::RowNotFound => "the data not exist",
                    _ => {
                        println!("{:#?}", err);
                        "database error"
                    }
                };
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }

            ApiError::RedisError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "cache error"),

            ApiError::ValidationError(_) => (StatusCode::BAD_REQUEST, "paramters validation error"),
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, "data not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
