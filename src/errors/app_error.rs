use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    ValidationError(String),
    InternalError(String),
    DatabaseError(sea_orm::DbErr),
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            ),
        };
        
        (status, message).into_response()
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::Unauthorized(msg) => write!(f, "{}", msg),
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::ValidationError(msg) => write!(f, "{}", msg),
            AppError::InternalError(msg) => write!(f, "{}", msg),
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}