use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use prisma_client_rust::QueryError;

#[derive(Debug, Clone)]
pub enum AppError {
    AnyhowError(SubError),
    PrismaError(SubError),
    SqlxError(SubError),
    GraphQLError(SubError),
    BadRequest(SubError),
    NotFound(SubError),
}

impl std::error::Error for AppError {}

#[derive(Debug, Clone)]
pub struct SubError {
    pub message: String,
}

impl SubError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::error::Error for SubError {}

impl Display for SubError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.message)
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::AnyhowError(SubError::new(error.to_string()))
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(serde::Serialize)]
        struct ErrorMessage {
            message: String,
            error_type: String,
            status_code: u16,
        }

        let error_type = match self {
            AppError::BadRequest(_) => "BadRequest",
            AppError::NotFound(_) => "NotFound",
            _ => "InternalServerError",
        };

        let status_code = match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match self {
            AppError::AnyhowError(e) => e.message,
            AppError::PrismaError(e) => e.message,
            AppError::GraphQLError(e) => e.message,
            AppError::SqlxError(e) => e.message,
            AppError::BadRequest(e) => e.message,
            AppError::NotFound(e) => e.message,
        };

        let json = Json(ErrorMessage {
            message,
            error_type: error_type.to_string(),
            status_code: status_code.as_u16(),
        });

        (status_code, json).into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sub_error = match self {
            AppError::AnyhowError(e) => Some(e),
            AppError::PrismaError(e) => Some(e),
            AppError::GraphQLError(e) => Some(e),
            AppError::SqlxError(e) => Some(e),
            AppError::BadRequest(e) => Some(e),
            AppError::NotFound(e) => Some(e),
        };

        if let Some(sub_error) = sub_error {
            write!(f, "{}", sub_error.message)
        } else {
            write!(f, "{:?}", self)
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::AnyhowError(SubError::new(error.to_string()))
    }
}

impl From<async_graphql::Error> for AppError {
    fn from(error: async_graphql::Error) -> Self {
        AppError::GraphQLError(SubError::new(format!("{:#?}", error)))
    }
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        AppError::PrismaError(SubError::new(error.to_string()))
    }
}

impl From<anyhow::Error> for SubError {
    fn from(error: anyhow::Error) -> Self {
        SubError::new(error.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::SqlxError(SubError::new(error.to_string()))
    }
}
