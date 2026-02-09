use axum::{extract::Request, middleware::Next};
use tracing::{info, instrument};
use uuid::Uuid;

/// We can define a middleware that logs <something>.
#[instrument(name = "log_middleware", fields(log_id=%Uuid::now_v7()))]
pub async fn log_middleware(request: Request, next: Next) {
    info!("uri: {:?}", &request.uri());
    let response = next.run(request).await;
    info!("response status: {:?}", response.status());
}
