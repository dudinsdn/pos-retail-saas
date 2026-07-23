use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal server error")]
    Internal,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Resource not found")]
    NotFound,

    #[error("Database error")]
    Database,

    #[error("Redis error")]
    Redis,

    #[error("Authentication failed")]
    Authentication,

    #[error("Token is invalid")]
    InvalidToken,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Config(_) | Self::Database | Self::Redis | Self::Internal => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized | Self::Authentication | Self::InvalidToken => {
                StatusCode::UNAUTHORIZED
            }
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
        };

        (status, self.to_string()).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(_: std::io::Error) -> Self {
        Self::Internal
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        Self::Config(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}

impl From<redis::RedisError> for AppError {
    fn from(_: redis::RedisError) -> Self {
        Self::Redis
    }
}
