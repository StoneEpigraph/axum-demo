use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::api::jwt::AuthError;

pub mod user;
mod jwt;
pub mod counter;

pub enum ApiError {
    Internal(anyhow::Error),
    Auth(AuthError),
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        ApiError::Internal(err.into())
    }
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        ApiError::Auth(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Auth(err) => err.into_response(),
            ApiError::Internal(err) => {
                let body = Json(json!({
                    "error": err.to_string(),
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}