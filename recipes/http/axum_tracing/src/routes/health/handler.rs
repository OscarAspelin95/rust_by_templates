use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::time::Duration;
use tracing::info;
use tracing::instrument;

#[instrument(name = "health_handler")]
pub async fn handler() -> Response {
    info!("Hello");
    tokio::time::sleep(Duration::from_secs(2)).await;
    (StatusCode::OK, "OK").into_response()
}
