use crate::common::types::BoxError;

use axum::{
    extract::{FromRequest, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(FromRequest)]
#[from_request(via(Json), rejection(ApiError))]
struct ApiResponse<T>(T);

impl<T> IntoResponse for ApiResponse<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest,
    RequestTimeout,
    ProcessFailed(BoxError),
    DbAccessFailed(sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            ApiError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request".to_owned()),
            ApiError::RequestTimeout => (StatusCode::REQUEST_TIMEOUT, "Request Timeout".to_owned()),
            ApiError::ProcessFailed(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error {err}").to_owned(),
            ),
            ApiError::DbAccessFailed(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error {err}").to_owned(),
            ),
        };

        (status, ApiResponse(ErrorResponse { message })).into_response()
    }
}

impl From<BoxError> for ApiError {
    fn from(error: BoxError) -> Self {
        Self::ProcessFailed(error)
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        Self::DbAccessFailed(error)
    }
}
