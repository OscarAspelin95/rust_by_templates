use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn handler() -> Response {
    (StatusCode::OK, "OK").into_response()
}
