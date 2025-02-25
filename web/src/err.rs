use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use serde_derive::Serialize;

use crate::extractors::AppJson;

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        Self::JsonRejection(value)
    }
}

// Handle errors
pub enum AppError {
    JsonRejection(JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        // custom json error message
        // otherwise defaults to 429: 'field' invalid type
        let (_status, message) = match self {
            AppError::JsonRejection(err) => {
                tracing::error!(%err, "Err parsing JSON input");
                (err.status(), err.body_text())
            }
        };

        (StatusCode::BAD_REQUEST, AppJson(ErrorResponse { message })).into_response()
    }
}
