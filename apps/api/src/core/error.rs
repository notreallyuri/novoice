use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Invalid request: {0}")]
    BadRequest(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Not found")]
    NotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Too many requests")]
    TooManyRequests,
}

impl AppError {
    fn get_codes(&self) -> (StatusCode, &'static str, String) {
        match self {
            Self::Database(msg) => {
                tracing::error!("Database Error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "Erro de persistência de dados".into(),
                )
            }
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            Self::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "AUTH_INVALID_CREDENTIALS",
                self.to_string(),
            ),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "AUTH_REQUIRED",
                "Autenticação necessária para acessar este recurso".into(),
            ),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, "AUTH_FORBIDDEN", msg.clone()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, "RESOURCE_CONFLICT", msg.clone()),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                "RESOURCE_NOT_FOUND",
                "Recurso não encontrado".into(),
            ),
            Self::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.clone())
            }
            Self::RedisError(err) => {
                tracing::error!("Redis Error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "CACHE_ERROR",
                    "Erro no serviço de sessão".into(),
                )
            }
            Self::Internal(err) => {
                tracing::error!("Internal Panic: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Um erro inesperado ocorreu".into(),
                )
            }
            Self::TooManyRequests => {
                tracing::error!("Rate limit exceeded");
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    "RATE_LIMIT_EXCEEDED",
                    "Muitas requisições. Tente novamente em 1 minuto.".into(),
                )
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = self.get_codes();
        let body = Json(json!({
            "success": false,
            "error": { "code": code, "message": message }
        }));
        (status, body).into_response()
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<scylla::errors::ExecutionError> for AppError {
    fn from(err: scylla::errors::ExecutionError) -> Self {
        Self::Database(format!("Scylla Execution: {}", err))
    }
}

impl From<scylla::errors::RowsError> for AppError {
    fn from(err: scylla::errors::RowsError) -> Self {
        Self::Database(format!("Scylla Rows: {}", err))
    }
}

impl From<scylla::errors::IntoRowsResultError> for AppError {
    fn from(err: scylla::errors::IntoRowsResultError) -> Self {
        Self::Database(format!("Scylla Result: {}", err))
    }
}

impl From<scylla::deserialize::DeserializationError> for AppError {
    fn from(err: scylla::deserialize::DeserializationError) -> Self {
        Self::Database(format!("Scylla Deserialization: {}", err))
    }
}

impl From<scylla::errors::PrepareError> for AppError {
    fn from(err: scylla::errors::PrepareError) -> Self {
        Self::Database(format!("Scylla Prepare: {}", err))
    }
}

impl From<scylla::errors::NewSessionError> for AppError {
    fn from(err: scylla::errors::NewSessionError) -> Self {
        Self::Database(format!("Scylla Session: {}", err))
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        Self::BadRequest(format!("Invalid UUID format: {}", err))
    }
}

impl From<deadpool_redis::CreatePoolError> for AppError {
    fn from(err: deadpool_redis::CreatePoolError) -> Self {
        Self::RedisError(err.to_string())
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        Self::RedisError(err.to_string())
    }
}

impl From<deadpool_redis::redis::RedisError> for AppError {
    fn from(err: deadpool_redis::redis::RedisError) -> Self {
        Self::RedisError(err.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::Internal(format!("Thread join error: {}", err))
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::Internal(format!("Password hashing error: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Internal(format!("JSON serialization error: {}", err))
    }
}

impl<E> From<aws_sdk_s3::error::SdkError<E>> for AppError
where
    E: std::fmt::Debug,
{
    fn from(err: aws_sdk_s3::error::SdkError<E>) -> Self {
        Self::Internal(format!("AWS S3 Error: {:?}", err))
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::ValidationError(err.to_string())
    }
}
