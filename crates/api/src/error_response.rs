use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use application::errors::ApplicationError;
use domain::errors::DomainError;

/// JSON body returned for any error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Newtype wrapper allowing `ApplicationError` to be turned into an
/// Axum `Response`, mapping each error variant to an appropriate HTTP
/// status code.
pub struct ApiError(pub ApplicationError);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self.0 {
            ApplicationError::Domain(DomainError::NotFound(_)) => StatusCode::NOT_FOUND,
            ApplicationError::Domain(DomainError::Validation(_)) => StatusCode::BAD_REQUEST,
            ApplicationError::Domain(DomainError::Conflict(_)) => StatusCode::CONFLICT,
            ApplicationError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            ApplicationError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = ErrorResponse {
            error: self.0.to_string(),
        };

        (status, Json(body)).into_response()
    }
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        Self(error)
    }
}
